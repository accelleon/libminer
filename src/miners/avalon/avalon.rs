use async_trait::async_trait;
use serde_json::json;
use lazy_regex::regex;

use crate::miner::{Miner, Pool};
use crate::miners::avalon::cgminer;
use crate::error::Error;
use crate::Client;

pub struct Avalon {
    ip: String,
    port: u16,
    username: String,
    password: String,
    client: Client,
}

#[async_trait]
impl Miner for Avalon {
    fn new(client: Client, ip: String, port: u16) -> Self {
        Avalon {
            ip,
            port,
            username: "".to_string(),
            password: "".to_string(),
            client,
        }
    }

    fn get_type(&self) -> &'static str {
        "Avalon"
    } 

    async fn get_model(&self) -> Result<String, Error> {
        let cmd = r#"{"command":"version"}"#;
        let resp = self.client.send_recv(&self.ip, self.port, &cmd).await?;
        let version = serde_json::from_str::<cgminer::VersionResp>(&resp)?;
        if let Some(version) = version.version {
            if let Some(version) = version.get(0) {
                Ok(version.model()?.to_string())
            } else {
                Err(Error::ApiCallFailed("version".to_string()))
            }
        } else {
            Err(Error::ApiCallFailed("version".to_string()))
        }
    }

    async fn auth(&mut self, username: &str, password: &str) -> Result<(), Error> {
        self.username = username.to_string();
        self.password = password.to_string();
        Ok(())
    }

    async fn reboot(&mut self) -> Result<(), Error> {
        let cmd = json!({
            "command": "ascset",
            "parameter": "0,reboot,0"
        });
        self.client.send(&self.ip, self.port, &cmd).await
    }

    async fn get_hashrate(&self) -> Result<f64, Error> {
        let cmd = r#"{"command":"estats"}"#;
        let resp = self.client.send_recv(&self.ip, self.port, &cmd).await?;
        let estats = cgminer::EStats::try_from(&serde_json::from_str::<cgminer::StatsResp>(&resp)?)?;
        Ok(estats.ghs_mm / 1000.0)
    }

    async fn get_nameplate_rate(&self) -> Result<f64, Error> {
        let cmd = r#"{"command":"version"}"#;
        let resp = self.client.send_recv(&self.ip, self.port, &cmd).await?;
        let version = serde_json::from_str::<cgminer::VersionResp>(&resp)?;
        if let Some(version) = version.version {
            if let Some(version) = version.get(0) {
                Ok(version.hashrate_th()?)
            } else {
                Err(Error::ApiCallFailed("version".to_string()))
            }
        } else {
            Err(Error::ApiCallFailed("version".to_string()))
        }
    }

    async fn get_temperature(&self) -> Result<f64, Error> {
        let cmd = r#"{"command":"estats"}"#;
        let resp = self.client.send_recv(&self.ip, self.port, &cmd).await?;
        let estats = cgminer::EStats::try_from(&serde_json::from_str::<cgminer::StatsResp>(&resp)?)?;
        Ok(estats.temp as f64)
    }

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error> {
        let cmd = r#"{"command":"estats"}"#;
        let resp = self.client.send_recv(&self.ip, self.port, &cmd).await?;
        let estats = cgminer::EStats::try_from(&serde_json::from_str::<cgminer::StatsResp>(&resp)?)?;
        Ok(vec![
            estats.fan1,
            estats.fan2,
            estats.fan3,
            estats.fan4,
        ])
    }

    async fn get_pools(&self) -> Result<Vec<Pool>, Error> {
        Err(Error::NotSupported)
    }

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error> {
        Err(Error::NotSupported)
    }

    async fn get_sleep(&self) -> Result<bool, Error> {
        let cmd = cgminer::PowerSupplyInfo::get_cmd().to_string();
        let resp = self.client.send_recv(&self.ip, self.port, &cmd).await?;
        let asc_hashpower = cgminer::PowerSupplyInfo::try_from(serde_json::from_str::<cgminer::StatusResp>(&resp)?)?;
        Ok(asc_hashpower.power == 0)
    }

    async fn set_sleep(&mut self, sleep: bool) -> Result<(), Error> {
        if sleep {
            let cmd = cgminer::PowerSupplyInfo::set_cmd(sleep).to_string();
            let s = self.client.send_recv(&self.ip, self.port, &cmd).await?;
            println!("set_sleep: {}", s);
            let status: cgminer::StatusResp = serde_json::from_str(&s)?;
            if status.status[0].status == cgminer::StatusCode::INFO {
                Ok(())
            } else {
                Err(Error::ApiCallFailed(status.status[0].msg.clone()))
            }
        } else {
            // If we're waking up, we need to reboot the miner
            self.reboot().await
        }
    }

    async fn get_blink(&self) -> Result<bool, Error> {
        let cmd = r#"{"command":"ascset","parameter":"0,led,1-255"}"#;
        let resp = self.client.send_recv(&self.ip, self.port, &cmd).await?;
        let status = serde_json::from_str::<cgminer::StatusResp>(&resp)?;
        if status.status[0].status == cgminer::StatusCode::INFO {
            let re = regex!(r#"LED\[(\d)\]"#);
            let caps = re.captures(&status.status[0].msg).ok_or(Error::InvalidResponse)?;
            let led = caps.get(1).unwrap().as_str().parse::<u8>().map_err(|_| Error::InvalidResponse)?;
            Ok(led > 0)
        } else {
            Err(Error::ApiCallFailed(status.status[0].msg.clone()))
        }
    }

    async fn set_blink(&mut self, blink: bool) -> Result<(), Error> {
        let cmd = match blink {
            true => r#"{"command":"ascset","parameter":"0,led,1"}"#,
            false => r#"{"command":"ascset","parameter":"0,led,0"}"#,
        };
        let resp = self.client.send_recv(&self.ip, self.port, &cmd).await?;
        let status = serde_json::from_str::<cgminer::StatusResp>(&resp)?;
        if status.status[0].status == cgminer::StatusCode::SUCC {
            Ok(())
        } else {
            Err(Error::ApiCallFailed(status.status[0].msg.clone()))
        }
    }

    async fn get_logs(&mut self) -> Result<Vec<String>, Error> {
        Err(Error::NotSupported)
    }

    async fn get_mac(&self) -> Result<String, Error> {
        let cmd = r#"{"command":"version"}"#;
        let resp = self.client.send_recv(&self.ip, self.port, &cmd).await?;
        let version = serde_json::from_str::<cgminer::VersionResp>(&resp)?;
        if let Some(version) = version.version {
            if let Some(version) = version.get(0) {
                Ok(version.mac_addr())
            } else {
                Err(Error::ApiCallFailed("version".to_string()))
            }
        } else {
            Err(Error::ApiCallFailed("version".to_string()))
        }
    }

    async fn get_errors(&mut self) -> Result<Vec<String>, Error> {
        Err(Error::NotSupported)
    }
}
