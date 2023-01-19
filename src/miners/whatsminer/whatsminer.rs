use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;
use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};
use lazy_regex::regex;
use std::collections::HashSet;
use crate::{Client, Miner, error::Error, Pool, miners::common, miners::whatsminer::wmapi};

use super::{error::WhatsminerErrors, wmapi::StatusCode};


#[derive(Debug, Deserialize)]
pub struct LogLen {
    pub logfilelen: String,
}

#[derive(Debug, Deserialize)]
pub struct LogsResponse {
    #[serde(rename = "STATUS")]
    pub status: common::StatusCode,
    #[serde(rename = "When")]
    pub when: usize,
    #[serde(rename = "Code")]
    pub code: usize,
    #[serde(rename = "Msg")]
    pub msg: Option<LogLen>,
    #[serde(rename = "Description")]
    pub description: String,
}

pub struct Whatsminer {
    ip: String,
    port: u16,
    password: Option<String>,
    token: Option<wmapi::WhatsminerToken>,
    client: Client,
}

impl Whatsminer {
    async fn send_recv<T>(&self, data: &T) -> Result<String, Error>
        where T: ToString
    {
        let mut resp = self.client.send_recv(&self.ip, self.port, data).await?;
        // Whatsminer can return non-compliant JSON
        resp = resp.replace("inf", "\"inf\"");
        resp = resp.replace("nan", "\"nan\"");
        resp = resp.replace(",}", "}");
        Ok(resp)
    }

    async fn refresh_token(&mut self) -> Result<(), Error> {
        if let Some(passwd) = &self.password {
            let resp = self.send_recv(&json!({"cmd": "get_token"})).await?;
            match serde_json::from_str::<wmapi::TokenResponse>(&resp) {
                Ok(token_resp) => {
                        self.token = Some(
                        token_resp
                        .make_token(passwd)
                        .map_err(|_| Error::ApiCallFailed("Failed to make token".into()))?
                    );
                    Ok(())
                },
                Err(e) => Err(e.into())
            }
        } else {
            Err(Error::Unauthorized)
        }
    }

    async fn send_recv_enc(&mut self, mut data: serde_json::Value) -> Result<String, Error> {
        if let Some(token) = &self.token {
            // Refresh our token if its expired
            if token.is_expired() {
                self.refresh_token().await?;
            }
            // We need to reborrow the token due to the possibility of it being mutated by the refresh
            // Should never panic so unwrap() is fine
            let token = self.token.as_ref().unwrap();
            // Stuff our token into the JSON
            data.as_object_mut().unwrap().insert("token".to_string(), serde_json::Value::String(token.get_token().into()));
            let enc_data = token.encrypt(&data)?;
            let resp = self.send_recv(&enc_data).await?;
            let js = serde_json::from_str(&resp).map_err(|_| Error::ApiCallFailed("Failed to parse JSON".into()))?;
            let dec_data = token.decrypt(&js)?;
            Ok(dec_data.to_string())
        } else {
            Err(Error::Unauthorized)
        }
    }
}

#[async_trait]
impl Miner for Whatsminer {
    fn new(client: Client, ip: String, port: u16) -> Self {
        Self {
            ip: ip.clone(),
            port,
            password: None,
            token: None,
            client,
        }
    }

    fn get_type(&self) -> &'static str {
        "Whatsminer"
    }

    async fn get_model(&self) -> Result<String, Error> {
        let resp = self.client.http_client
            .get(format!("https://{}/cgi-bin/luci/admin/status/overview", self.ip))
            .send()
            .await?
            .text()
            .await?;
        let modelre = regex!(r#"<td.+>Model</td>\s*<td>WhatsMiner ([a-zA-Z0-9]+)(?:_V.+)?</td>"#);
        let model = modelre.captures(&resp)
            .ok_or(Error::ExpectedReturn)?
            .get(1)
            .ok_or(Error::ExpectedReturn)?
            .as_str();
        Ok(model.to_string())
    }

    async fn auth(&mut self, username: &str, password: &str) -> Result<(), Error> {
        self.password = Some(password.to_string());
        let r = self.client.http_client
            .post(format!("https://{}/cgi-bin/luci", self.ip))
            .form(&[("luci_username", username), ("luci_password", password)])
            .send()
            .await?;
        if r.status() != 200 {
            return Err(Error::Unauthorized);
        }
        self.refresh_token().await?;
        Ok(())
    }

    async fn reboot(&mut self) -> Result<(), Error> {
        let js = json!({
            "command": "reboot",
        });
        let resp = self.send_recv_enc(js).await?;
        Ok(())
    }

    async fn get_hashrate(&self) -> Result<f64, Error> {
        let resp = self.send_recv(&json!({"cmd":"summary"})).await?;
        if let Ok(status) = serde_json::from_str::<wmapi::Status>(&resp) {
            // We could error or assume not hashing
            // Err(Error::ApiCallFailed(status.msg))
            Ok(0.0)
        } else {
            let sum: wmapi::SummaryResp = serde_json::from_str(&resp)?;
            Ok(sum.summary[0].hs_rt / 1000000.0)
        }
    }

    async fn get_power(&self) -> Result<f64, Error> {
        let resp = self.send_recv(&json!({"cmd":"summary"})).await?;
        let sum: wmapi::SummaryResp = serde_json::from_str(&resp)?;
        Ok(sum.summary[0].power as f64)
    }

    async fn get_nameplate_rate(&self) -> Result<f64, Error> {
        let resp = self.send_recv(&json!({"cmd":"summary"})).await?;
        let hash: wmapi::SummaryResp = serde_json::from_str(&resp)?;
        Ok(hash.summary[0].factory_ghs as f64 / 1000.0)
    }

    async fn get_temperature(&self) -> Result<f64, Error> {
        let resp = self.send_recv(&json!({"cmd":"summary"})).await?;
        let sum: wmapi::SummaryResp = serde_json::from_str(&resp)?;
        Ok(sum.summary[0].temperature)
    }

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error> {
        let resp = self.send_recv(&json!({"cmd":"summary"})).await?;
        let sum: wmapi::SummaryResp = serde_json::from_str(&resp)?;
        Ok(vec![sum.summary[0].fan_speed_in, sum.summary[0].fan_speed_out])
    }

    async fn get_pools(&self) -> Result<Vec<Pool>, Error> {
        let resp = self.send_recv(&json!({"cmd":"pools"})).await?;
        let pools: common::PoolsResp = serde_json::from_str(&resp)?;
        Ok(pools.pools.iter().map(|p| Pool {
            url: p.url.clone(),
            username: p.user.clone(),
            password: None,
        }).collect())
    }

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error> {
        //TODO: this can panic
        let js = json!({
            "cmd": "update_pools",
            "pool1": pools[0].url,
            "worker1": pools[0].username,
            "passwd1": pools[0].password,
            "pool2": pools[1].url,
            "worker2": pools[1].username,
            "passwd2": pools[1].password,
            "pool3": pools[2].url,
            "worker3": pools[2].username,
            "passwd3": pools[2].password,
        });
        let resp = self.send_recv_enc(js).await?;
        //println!("{}", resp);
        Ok(())
    }

    async fn get_sleep(&self) -> Result<bool, Error> {
        //This doesn't work for miners running cgminer
        let resp = self.send_recv(&json!({"cmd":"status"})).await?;
        let btstatus: wmapi::BtStatusResp = serde_json::from_str(&resp)?;
        match btstatus.msg.btmineroff {
            false => Ok(false),
            true => {
                // Double check that cgminer isn't running
                // Scrape the web API yet again
                let r = self.client.http_client
                .get(&format!("https://{}/cgi-bin/luci/admin/status/processes", self.ip))
                    .send()
                    .await?
                    .text()
                    .await?;
                let re = regex!(r#".COMMAND" value="(cg|bt)miner" />"#);
                Ok(!re.is_match(&r))
            }
        }
    }

    async fn set_sleep(&mut self, sleep: bool) -> Result<(), Error> {
        let js = match sleep {
            true => json!({
                "cmd": "power_off",
                "respbefore": "true", // Please respond before power off
            }),
            false => json!({
                "cmd": "power_on",
            }),
        };
        let resp = self.send_recv_enc(js).await?;
        let stat = serde_json::from_str::<wmapi::Status>(&resp)?;
        if stat.status == StatusCode::SUCC {
            Ok(())
        } else {
            Err(Error::ApiCallFailed(stat.msg))
        }
    }

    async fn get_blink(&self) -> Result<bool, Error> {
        let resp = self.send_recv(&json!({"cmd":"get_miner_info"})).await?;
        if let Ok(status) = serde_json::from_str::<wmapi::Status>(&resp) {
            // We could error or assume not hashing
            // Err(Error::ApiCallFailed(status.msg))
            Ok(false)
        } else {
            let resp: wmapi::MinerInfoResponse = serde_json::from_str(&resp)?;
            Ok(resp.msg.ledstat != "auto")
        }
    }

    async fn set_blink(&mut self, blink: bool) -> Result<(), Error> {
        let js = match blink {
            true => json!({
                "command": "set_led",
                "color": "red",
                "period": 1000,
                "duration": 500,
                "start": 0,
            }),
            false => json!({
                "command": "set_led",
                "param": "auto",
            }),
        };
        let resp = self.send_recv_enc(js).await?;
        //println!("{}", resp);
        Ok(())
    }

    async fn get_logs(&mut self) -> Result<Vec<String>, Error> {
        if let Some(token) = &self.token {
            let js = token.encrypt(&json!({
                "command": "download_logs",
                "token": token.get_token(),
            }))?;
            // This responds in 2 parts, the first part is a status response for the command
            // the second part is the logs sent 10ms after the first part.
            let mut stream = TcpStream::connect(format!("{}:{}", &self.ip, self.port)).await?;
            stream.writable().await?;
            stream.write_all(js.to_string().as_bytes()).await?;
            let mut resp = String::new();
            stream.readable().await?;
            stream.read_to_string(&mut resp).await?;
            resp = resp.replace("\0", "");
            
            let status: LogsResponse = serde_json::from_str(&resp)?;
            if status.status == common::StatusCode::SUCC {
                let mut resp = String::new();
                stream.readable().await?;
                stream.read_to_string(&mut resp).await?;
                resp = resp.replace("\0", "");
                Ok(resp.split('\n').map(|s| s.to_string()).collect())
            } else {
                //println!("Failed to get logs");
                Err(Error::Unauthorized)
            }
        } else {
            Err(Error::Unauthorized)
        }
    }

    async fn get_mac(&self) -> Result<String, Error> {
        let resp = self.send_recv(&json!({"cmd":"get_miner_info"})).await?;
        if let Ok(_) = serde_json::from_str::<wmapi::Status>(&resp) {
            // Older API version
            let resp = self.send_recv(&json!({"cmd":"summary"})).await?;
            let resp: wmapi::SummaryResp = serde_json::from_str(&resp)?;
            resp.summary[0].mac.clone().ok_or(Error::ApiCallFailed("Failed to get MAC".to_string()))
        } else {
            let resp: wmapi::MinerInfoResponse = serde_json::from_str(&resp)?;
            Ok(resp.msg.mac.clone())
        }
    }

    async fn get_errors(&mut self) -> Result<Vec<String>, Error> {
        let resp = self.send_recv(&json!({"cmd":"get_error_code"})).await?;
        // Whatsminer again returning invalid JSON
        //{"error_code":["111":"2022-10-20 09:18:54","110":"2022-10-20 09:18:54","2010":"1970-01-02 08:00:04"]}
        //TODO: it might be cheaper to regex this
        let resp = resp.replace("[", "{").replace("]", "}");
        let resp = serde_json::from_str::<wmapi::ErrorResp>(&resp)?;
        // Our response is a hashmap of error_code : datetime
        // I only care about the error codes, throw them into a single string to regex against
        let log = resp.msg.error_code.keys()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let mut errors = HashSet::new();
        for err in WhatsminerErrors.iter() {
            if let Some(msg) = err.get_msg(&log) {
                errors.insert(msg);
            }
        }
        Ok(errors.into_iter().collect())
    }
}
