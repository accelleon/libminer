mod util;
pub mod miners;
mod miner;

pub use miner::{Miner, Pool};
pub mod error;

use miners::*;
use error::Error;
use tokio::{self, net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};
use reqwest;
use serde_json::json;
use tracing::{debug, warn, instrument};
pub use tokio::time::Duration;
use lazy_regex::regex;

/*
 * Cgminer socket API has a tendency to fail often but is generally universal
 * Failing this, most miners have an API exposed over HTTP, but these are highly specific
 */

pub struct ClientBuilder {
    connect_timeout: Duration,
    request_timeout: Duration,
}

impl ClientBuilder {
    pub fn new () -> Self {
        Self {
            connect_timeout: Duration::from_secs(5),
            request_timeout: Duration::from_secs(10),
        }
    }

    /// Set the connect timeout for the client
    /// Default is 5 seconds
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Set the request timeout for the client
    /// Default is 10 seconds
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    pub fn build(self) -> Result<Client, Error> {
        let client = reqwest::ClientBuilder::new()
            .user_agent("libminer/0.1")
            .connect_timeout(self.connect_timeout)
            .timeout(self.request_timeout)
            //.tcp_keepalive(None)
            .tcp_nodelay(true) // Disable Nagle's algorithm, which can cause latency issues
            .danger_accept_invalid_certs(true) // Accept self-signed certs
            .cookie_store(true) // Some miners require a cookie store
            .build()?;
        Ok(Client {
            http_client: client,
            connect_timeout: self.connect_timeout,
            request_timeout: self.request_timeout,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    http_client: reqwest::Client,
    connect_timeout: Duration,
    request_timeout: Duration,
}

impl Client {
    /// Connect to a given host with the timeout specified
    async fn connect(&self, ip: &str, port: u16) -> Result<TcpStream, Error> {
        match tokio::time::timeout(
            self.connect_timeout,
            TcpStream::connect(format!("{}:{}", ip, port))
        ).await {
            Ok(stream_result) => Ok(stream_result?),
            Err(_) => Err(Error::Timeout),
        }
    }

    /// Connect to a host and send data return data as String, close connection after request
    async fn send_recv<T>(&self, ip: &str, port: u16, data: &T) -> Result<String, Error> 
        where T: ToString
    {
        let mut stream = self.connect(ip, port).await?;
        match tokio::time::timeout(
            self.request_timeout,
            async {
                stream.writable().await?;
                stream.write_all(data.to_string().as_bytes()).await?;
                let mut buf = String::new();
                stream.readable().await?;
                stream.read_to_string(&mut buf).await?;
                buf = buf.replace("\0", ""); // Fix for Antminer bug
                Ok(buf)
            }
        ).await {
            Ok(result) => result,
            Err(_) => Err(Error::Timeout),
        }
    }

    /// Attempts to perform miner detection against the cgminer socket API roughly implemented by most miners
    /// NOTES:
    /// * On Minervas using the Minera interface, the cgminer API can be deadlocked
    /// * On Whatsminers, the socket API can be responsive but btminer deadlocked, this results in detection successful but every call failing
    async fn socket_detect(&self, ip: &str, port: u16) -> Result<Box<dyn Miner + Send + Sync>, Error> {
        debug!("Trying socket detection...");
        match self.send_recv(ip, port, &json!({"command": "stats"})).await {
            Ok(resp) => {
                debug!("Received response from socket API...");
                if let Ok(stats_resp) = serde_json::from_str::<common::StatsResp>(&resp) {
                    debug!("Received valid cgminer response.");
                    if stats_resp.status[0].status != common::StatusCode::SUCC {
                        return Err(Error::ApiCallFailed(stats_resp.status[0].msg.clone()));
                    }
                    if let Some(stats) = stats_resp.stats {
                        debug!("Checking for type in stats response...");
                        for stat in stats {
                            match stat {
                                #[cfg(feature = "antminer")]
                                common::Stats::AmVersion(_) => {
                                    debug!("Found Antminer miner at {}", ip);
                                    return Ok(Box::new(antminer::Antminer::new(self.clone(), ip.into(), port)));
                                },
                                #[cfg(feature = "minerva")]
                                common::Stats::Dev(stat) => {
                                    if let Some(type_) = stat.type_ {
                                        if type_ == "Minerva" {
                                            // We need to differentiate between the 2 interfaces
                                            // easiest thing is to send a GET request to /index.php
                                            // If we get a 200, we know its running minera
                                            debug!("Found Minerva, determining interface...");
                                            let resp2 = self.http_client
                                                .get(&format!("http://{}/index.php", ip))
                                                .send()
                                                .await?;
                                            return match resp2.status() {
                                                reqwest::StatusCode::NOT_FOUND => {
                                                    debug!("Found Minerva (Custom Interface) at {}", ip);
                                                    Ok(Box::new(minerva::Minerva::new(self.clone(), ip.into(), port)))
                                                }
                                                reqwest::StatusCode::OK => {
                                                    debug!("Found Minerva (Minera Interface) at {}", ip);
                                                    Ok(Box::new(minerva::Minera::new(self.clone(), ip.into(), port)))
                                                }
                                                _ => {
                                                    debug!("Unable to determine interface for Minerva at {}", ip);
                                                    Err(Error::UnknownMinerType)
                                                },
                                            };
                                        } else {
                                            debug!("Unsupported miner type: {} at {}", type_, ip);
                                            return Err(Error::UnknownMinerType);
                                        }
                                    } else {
                                        debug!("Miner did not include type in response at {}", ip);
                                        return Err(Error::UnknownMinerType);
                                    }
                                }
                                _ => {} // We don't care about the other stats
                            }
                        }
                        debug!("Stats did not include a section containing type at {}\n{}", ip, resp);
                        return Err(Error::UnknownMinerType);
                    } else {
                        debug!("Unable to parse stats response at {}\n{}", ip, resp);
                        return Err(Error::UnknownMinerType);
                    }
                } else if let Ok(status) = serde_json::from_str::<common::Status>(&resp) {
                    // Whatsminer returns just the cgminer status error with invalid json and a description containing whatsminer
                    // {"STATUS":"E","When":"0","Code":23,"Msg":"Invalid JSON","Description":"whatsminer"}
                    //TODO: Don't hardcode the status code for Invalid Command
                    #[cfg(feature = "whatsminer")]
                    if status.status == common::StatusCode::ERROR && status.code == 14 {
                        // lowercase and regex the description for "whatsminer"
                        if let Some(desc) = status.description {
                            if desc.to_lowercase().contains("whatsminer") {
                                debug!("Found Whatsminer at {}", ip);
                                return Ok(Box::new(whatsminer::Whatsminer::new(self.clone(), ip.into(), port)));
                            }
                        }
                    }
                    debug!("Received error response but not whatsminer at {}\n{}", ip, resp);
                    return Err(Error::UnknownMinerType);
                } else {
                    debug!("Unable to parse response from socket API: {}", resp);
                    return Err(Error::UnknownMinerType);
                }
            }
            Err(e) => {
                debug!("Error while sending request to socket API: {}", e);
                return Err(Error::UnknownMinerType);
            }
        }
    }

    async fn http_detect(&self, ip: &str, port: u16) -> Result<Box<dyn Miner + Send + Sync>, Error> {
        debug!("Trying HTTP detection...");
        // To reduce traffic and since detection is entirely on status response, we can just send a HEAD request
        // Start with Antminer, if this fails to connect return a timeout
        match self.http_client.head(&format!("http://{}/", ip)).send().await {
            Ok(resp) => {
                debug!("Received response from HTTP API...");
                //TODO: In theory we could probably do this with a single request
                #[cfg(feature = "antminer")]
                if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
                    if let Some(auth) = resp.headers().get("WWW-Authenticate") {
                        let re = regex!(r"^[Dd]igest");
                        if re.is_match(auth.to_str().unwrap()) {
                            debug!("Found Antminer at {}", ip);
                            return Ok(Box::new(antminer::Antminer::new(self.clone(), ip.into(), port)));
                        }
                    }
                }

                #[cfg(feature = "minerva")]
                {
                    // 2 fan minervas have the title Minerva and are based off umi
                    debug!("Checking for custom Minerva...");
                    let re = regex!(r"Minerva(.|\n)+umi");
                    let resp = self.http_client.get(&format!("https://{}", ip)).send().await?;
                    let text = resp.text().await?;
                    if re.is_match(&text) {
                        debug!("Found Minerva (Custom Interface) at {}", ip);
                        return Ok(Box::new(minerva::Minerva::new(self.clone(), ip.into(), port)));
                    }

                    // 4 fan minervas permit a request to /index.php/app/stats even when not logged in
                    debug!("Checking for minera Minerva...");
                    let resp = self.http_client.head(&format!("http://{}/index.php/app/stats", ip)).send().await?;
                    if resp.status() == reqwest::StatusCode::OK {
                        debug!("Found Minerva at {}", ip);
                        return Ok(Box::new(minerva::Minera::new(self.clone(), ip.into(), port)));
                    }
                }
                
                #[cfg(feature = "whatsminer")]
                {
                    // Lastly check whatsminers, /cgi-bin/luci and look for whatsminer in the body
                    debug!("Checking for Whatsminer...");
                    let resp = self.http_client.get(&format!("http://{}/cgi-bin/luci", ip)).send().await?;
                    if resp.status() == reqwest::StatusCode::FORBIDDEN {
                        let re = regex!(r"<title>WhatsMiner");
                        if re.is_match(&resp.text().await?) {
                            debug!("Detected Whatsminer at {}:{}", ip, port);
                            warn!("Socket API did not respond, this miner may not work.");
                            return Ok(Box::new(whatsminer::Whatsminer::new(self.clone(), ip.to_string(), port)));
                        }
                    }
                }

                debug!("Unable to determine miner type {}", ip);
                Err(Error::UnknownMinerType)
            }
            Err(e) => {
                debug!("Error while sending request to HTTP API: {}", e);
                Err(Error::Timeout)
            }
        }
    }

    /// Detects the type of miner at the given IP and port
    /// Default port is 4028
    #[instrument]
    pub async fn get_miner(&self, ip: &str, port: Option<u16>) -> Result<Box<dyn Miner + Send + Sync>, Error> {
        let port = port.unwrap_or(4028);
        debug!("Detecting miner at {}:{}", ip, port);
        if let Ok(miner) = self.socket_detect(ip, port).await {
            Ok(miner)
        } else {
            self.http_detect(ip, port).await
        }
    }
}