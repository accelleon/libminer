use async_trait::async_trait;
use serde_json::json;

use crate::util::digest_auth::WithDigestAuth;
use crate::miner::{Miner, Pool};
use crate::miners::antminer::cgi;
use crate::error::Error;
use crate::Client;

use tracing::debug;

pub struct Antminer {
    ip: String,
    port: u16,
    username: String,
    password: String,
    model: String,
    client: Client,
}

#[async_trait]
impl Miner for Antminer {
    fn new(client: Client, ip: String, port: u16) -> Self {
        Antminer {
            ip,
            port,
            username: "".to_string(),
            password: "".to_string(),
            model: "".to_string(),
            client,
        }
    }

    fn get_type(&self) -> &'static str {
        "Antminer"
    }

    async fn get_model(&self) -> Result<String, Error> {
        let resp = self.client.http_client
            .get(&format!("http://{}/cgi-bin/get_system_info.cgi", self.ip))
            .send_with_digest_auth(&self.username, &self.password)
            .await?;
        if resp.status().is_success() {
            let sys_info: cgi::SystemInfoResponse = resp.json().await?;
            Ok(sys_info.minertype.replace("Antminer ", "").replace(" ", "").to_lowercase())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn auth(&mut self, username: &str, password: &str) -> Result<(), Error> {
        self.username = username.to_string();
        self.password = password.to_string();
        // Test authentication with a simple get request
        match self.client.http_client
            .get(&format!("http://{}/cgi-bin/get_miner_conf.cgi", self.ip))
            .send_with_digest_auth(&self.username, &self.password)
            .await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        Ok(())
                    } else {
                        Err(Error::Unauthorized)
                    }
                },
                Err(e) => Err(e.into()),
            }
    }

    async fn reboot(&mut self) -> Result<(), Error> {
        let resp = self.client.http_client
            .get(&format!("http://{}/cgi-bin/reboot.cgi", self.ip))
            .send_with_digest_auth(&self.username, &self.password)
            .await;
        // Miner reboots before a response is returned, so actually we want this to fail
        if let Err(_) = resp {
            Ok(())
        } else {
            Err(Error::ApiCallFailed("Reboot failed".to_string()))
        }
    }

    async fn get_hashrate(&self) -> Result<f64, Error> {
        let resp = self.client.http_client
            .get(&format!("http://{}/cgi-bin/summary.cgi", self.ip))
            .send_with_digest_auth(&self.username, &self.password)
            .await?;
        if resp.status().is_success() {
            //TODO: We should parse the status and properly return errors
            let text = resp.text().await?;
            //println!("response {}", text);
            let summary: cgi::SummaryResponse = serde_json::from_str(&text)?;
            if let Some(sum) = summary.summary.get(0) {
                Ok(sum.rate_5s / 1000.0)
            } else {
                // Miner can not return a summary if it is not mining
                Ok(0.0)
            }
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_nameplate_rate(&self) -> Result<f64, Error> {
        let resp = self.client.http_client
            .get(&format!("http://{}/cgi-bin/stats.cgi", self.ip))
            .send_with_digest_auth(&self.username, &self.password)
            .await?;
        if resp.status().is_success() {
            let stats = resp.json::<cgi::StatsResponse>().await;
            debug!("stats: {:?}", stats);
            let stats = stats?;
            if let Some(stat) = stats.stats.get(0) {
                Ok(stat.rate_ideal / 1000.0)
            } else {
                //TODO: Decide to return an error or just an empty vector
                Ok(0.0)
            }
        } else {
            //println!("{:?}", resp);
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_temperature(&self) -> Result<f64, Error> {
        // Antminer doesn't report a single temperature,
        // instead return the average of the board sensors
        let resp = self.client.http_client
            .get(&format!("http://{}/cgi-bin/stats.cgi", self.ip))
            .send_with_digest_auth(&self.username, &self.password)
            .await?;
        if resp.status().is_success() {
            let stats: cgi::StatsResponse = resp.json().await?;
            if let Some(stat) = stats.stats.get(0) {
                let mut ret = 0.0;
                let mut ntemp = 0;
                for chain in &stat.chain {
                    for temp in &chain.temp_pcb {
                        ntemp += 1;
                        ret += *temp as f64;
                    }
                }
                Ok(ret / ntemp as f64)
            } else {
                //TODO: Decide to return an error or just an empty vector
                Ok(0.0)
            }
        } else {
            //println!("{:?}", resp);
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error> {
        let resp = self.client.http_client
            .get(&format!("http://{}/cgi-bin/stats.cgi", self.ip))
            .send_with_digest_auth(&self.username, &self.password)
            .await?;
        if resp.status().is_success() {
            let stats: cgi::StatsResponse = resp.json().await?;
            if let Some(stat) = stats.stats.get(0) {
                //TODO: Gotta be a way to avoid this clone
                Ok(stat.fan.clone())
            } else {
                //TODO: Decide to return an error or just an empty vector
                Ok(vec![])
            }
        } else {
            //println!("{:?}", resp);
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_pools(&self) -> Result<Vec<Pool>, Error> {
        let resp = self.client.http_client
            .get(&format!("http://{}/cgi-bin/get_miner_conf.cgi", self.ip))
            .send_with_digest_auth(&self.username, &self.password)
            .await?;
        if resp.status().is_success() {
            let json = resp.json::<cgi::GetConfResponse>().await?;
            let pools = json.pools.into_iter().map(|pool| {
                Pool {
                    url: pool.url,
                    username: pool.user,
                    password: Some(pool.pass),
                }
            }).collect();
            Ok(pools)
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error> {
        let resp = self.client.http_client
            .post(&format!("http://{}/cgi-bin/set_miner_conf.cgi", self.ip))
            .json(&pools)
            .send_with_digest_auth(&self.username, &self.password)
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn set_sleep(&mut self, sleep: bool) -> Result<(), Error> {
        let resp = self.client.http_client
            .post(&format!("http://{}/cgi-bin/set_miner_conf.cgi", self.ip))
            .json(&json!({
                "miner-mode": sleep as u8,
            }))
            .send_with_digest_auth(&self.username, &self.password)
            .await?;
        if resp.status().is_success() {
            //println!("{}", resp.text().await?);
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn set_blink(&mut self, blink: bool) -> Result<(), Error> {
        let resp = self.client.http_client
            .post(&format!("http://{}/cgi-bin/blink.cgi", self.ip))
            .json(&json!({
                "blink": blink,
            }))
            .send_with_digest_auth(&self.username, &self.password)
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_logs(&mut self) -> Result<Vec<String>, Error> {
        unimplemented!()
    }

    async fn get_mac(&self) -> Result<String, Error> {
        let resp = self.client.http_client
        .get(&format!("http://{}/cgi-bin/get_system_info.cgi", self.ip))
        .send_with_digest_auth(&self.username, &self.password)
        .await?;
        if resp.status().is_success() {
            let sys_info: cgi::SystemInfoResponse = resp.json().await?;
            Ok(sys_info.macaddr)
        } else {
            Err(Error::HttpRequestFailed)
        }
    }
}