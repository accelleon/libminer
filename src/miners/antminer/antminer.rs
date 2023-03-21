use async_trait::async_trait;
use serde_json::json;
use std::{
    collections::HashSet,
};
use phf::phf_map;
use tokio::sync::{Mutex, MutexGuard};

use crate::util::digest_auth::WithDigestAuth;
use crate::miner::{Miner, Pool};
use crate::miners::antminer::cgi;
use crate::error::Error;
use crate::Client;
use crate::miners::antminer::error::AntminerErrors;

use super::cgi::SetConf;

/// Antminer models and their rated watt per TH
/// If more than 1 variant exists, this will be an average of all variants
/// Antminer rates these @25C
static POWER_MAP: phf::Map<&'static str, f64> = phf_map! {
    "t19" => 37.5,
    "s19" => 34.7,
    "s19j" => 34.5,
    "s19a" => 34.5,
    "s19pro" => 30.0,
    "s19jpro" => 29.5,
    "s19apro" => 29.5,
    "s19jpro+" => 27.5,
    "s19xp" => 22.0,
};

pub struct Antminer {
    ip: String,
    username: String,
    password: String,
    client: Client,

    sys_info: Mutex<Option<cgi::SystemInfoResponse>>,
    summary: Mutex<Option<cgi::SummaryResponse>>,
    miner_conf: Mutex<Option<cgi::GetConfResponse>>,
    stats: Mutex<Option<cgi::StatsResponse>>,
}

impl Antminer {
    async fn sys_info(&self) -> Result<MutexGuard<Option<cgi::SystemInfoResponse>>, Error> {
        let mut sys_info = self.sys_info.lock().await;
        if sys_info.is_none() {
            let resp = self.client.http_client
                .get(&format!("http://{}/cgi-bin/get_system_info.cgi", self.ip))
                .send_with_digest_auth(&self.username, &self.password)
                .await?;
            if !resp.status().is_success() {
                if resp.status().as_u16() == 401 {
                    return Err(Error::Unauthorized);
                }
                return Err(Error::HttpRequestFailed);
            }
            *sys_info = Some(resp.json().await?);
        }
        Ok(sys_info)
    }

    async fn summary(&self) -> Result<MutexGuard<Option<cgi::SummaryResponse>>, Error> {
        let mut summary = self.summary.lock().await;
        if summary.is_none() {
            let resp = self.client.http_client
                .get(&format!("http://{}/cgi-bin/summary.cgi", self.ip))
                .send_with_digest_auth(&self.username, &self.password)
                .await?;
            if !resp.status().is_success() {
                if resp.status().as_u16() == 401 {
                    return Err(Error::Unauthorized);
                }
                return Err(Error::HttpRequestFailed);
            }
            *summary = Some(resp.json().await?);
        }
        Ok(summary)
    }

    async fn miner_conf(&self) -> Result<MutexGuard<Option<cgi::GetConfResponse>>, Error> {
        let mut miner_conf = self.miner_conf.lock().await;
        if miner_conf.is_none() {
            let resp = self.client.http_client
                .get(&format!("http://{}/cgi-bin/get_miner_conf.cgi", self.ip))
                .send_with_digest_auth(&self.username, &self.password)
                .await?;
            if !resp.status().is_success() {
                if resp.status().as_u16() == 401 {
                    return Err(Error::Unauthorized);
                }
                return Err(Error::HttpRequestFailed);
            }
            *miner_conf = Some(resp.json().await?);
        }
        Ok(miner_conf)
    }

    async fn stats(&self) -> Result<MutexGuard<Option<cgi::StatsResponse>>, Error> {
        let mut stats = self.stats.lock().await;
        if stats.is_none() {
            let resp = self.client.http_client
                .get(&format!("http://{}/cgi-bin/stats.cgi", self.ip))
                .send_with_digest_auth(&self.username, &self.password)
                .await?;
            if !resp.status().is_success() {
                if resp.status().as_u16() == 401 {
                    return Err(Error::Unauthorized);
                }
                return Err(Error::HttpRequestFailed);
            }
            *stats = Some(resp.json().await?);
        }
        Ok(stats)
    }

    async fn invalidate(&self) {
        let _ = self.summary.lock().await.take();
        let _ = self.miner_conf.lock().await.take();
        let _ = self.stats.lock().await.take();
    }
}

#[async_trait]
impl Miner for Antminer {
    fn new(client: Client, ip: String, port: u16) -> Self {
        Antminer {
            ip,
            username: "".to_string(),
            password: "".to_string(),
            client,
            sys_info: Mutex::new(None),
            summary: Mutex::new(None),
            miner_conf: Mutex::new(None),
            stats: Mutex::new(None),
        }
    }

    fn get_type(&self) -> &'static str {
        "Antminer"
    }

    async fn get_model(&self) -> Result<String, Error> {
        let sys_info = self.sys_info().await?;
        let sys_info = sys_info.as_ref().unwrap_or_else(|| unreachable!());

        Ok(sys_info.minertype.replace("Antminer ", "").replace(" ", "").to_lowercase())
    }

    async fn auth(&mut self, username: &str, password: &str) -> Result<(), Error> {
        self.username = username.to_string();
        self.password = password.to_string();
        // Test authentication with a simple get request
        match self.sys_info().await {
                Ok(_) => Ok(()),
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
            self.invalidate().await;
            Ok(())
        } else {
            Err(Error::ApiCallFailed("Reboot failed".to_string()))
        }
    }

    async fn get_hashrate(&self) -> Result<f64, Error> {
        let summary = self.summary().await?;
        let summary = summary.as_ref().unwrap_or_else(|| unreachable!());

        if let Some(sum) = summary.summary.get(0) {
            Ok(sum.rate_5s / 1000.0)
        } else {
            // Miner can not return a summary if it is not mining
            Ok(0.0)
        }
    }

    async fn get_power(&self) -> Result<f64, Error> {
        match self.get_hashrate().await {
            Ok(hashrate) => {
                let model = self.get_model().await?;
                Ok(hashrate * POWER_MAP.get(model.as_str()).ok_or(Error::UnknownModel(model))?)
            },
            Err(e) => Err(e),
        }
    }

    async fn get_efficiency(&self) -> Result<f64, Error> {
        let model = self.get_model().await?;
        Ok(*POWER_MAP.get(model.as_str()).ok_or(Error::UnknownModel(model))?)
    }

    async fn get_nameplate_rate(&self) -> Result<f64, Error> {
        let stats = self.stats().await?;
        let stats = stats.as_ref().unwrap_or_else(|| unreachable!());

        if let Some(stat) = stats.stats.get(0) {
            Ok(stat.rate_ideal / 1000.0)
        } else {
            //TODO: Decide to return an error or just an empty vector
            Ok(0.0)
        }
    }

    async fn get_temperature(&self) -> Result<f64, Error> {
        // Antminer doesn't report a single temperature,
        // instead return the average of the chip sensors
        let stats = self.stats().await?;
        let stats = stats.as_ref().unwrap_or_else(|| unreachable!());

        if let Some(stat) = stats.stats.get(0) {
            let mut ret = 0.0;
            let mut ntemp = 0;
            for chain in &stat.chain {
                for temp in &chain.temp_chip {
                    ntemp += 1;
                    ret += *temp as f64;
                }
            }
            Ok(ret / ntemp as f64)
        } else {
            //TODO: Decide to return an error or just an empty vector
            Ok(0.0)
        }
    }

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error> {
        let stats = self.stats().await?;
        let stats = stats.as_ref().unwrap_or_else(|| unreachable!());

        if let Some(stat) = stats.stats.get(0) {
            //TODO: Gotta be a way to avoid this clone
            Ok(stat.fan.clone())
        } else {
            //TODO: Decide to return an error or just an empty vector
            Ok(vec![])
        }
    }

    async fn get_pools(&self) -> Result<Vec<Pool>, Error> {
        let miner_conf = self.miner_conf().await?;
        let miner_conf = miner_conf.as_ref().unwrap_or_else(|| unreachable!());

        Ok(miner_conf.pools.clone())
    }

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error> {
        let miner_conf = self.miner_conf().await?;
        let miner_conf = miner_conf.as_ref().unwrap_or_else(|| unreachable!());

        let mut json: SetConf = SetConf::from(miner_conf);
        json.pools = pools;

        let resp = self.client.http_client
            .post(&format!("http://{}/cgi-bin/set_miner_conf.cgi", self.ip))
            .json(&json)
            .send_with_digest_auth(&self.username, &self.password)
            .await?;
        if resp.status().is_success() {
            self.invalidate().await;
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_sleep(&self) -> Result<bool, Error> {
        let miner_conf = self.miner_conf().await?;
        let miner_conf = miner_conf.as_ref().unwrap_or_else(|| unreachable!());

        Ok(miner_conf.bitmain_work_mode == "1")
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
            self.invalidate().await;
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_blink(&self) -> Result<bool, Error> {
        let resp = self.client.http_client
            .get(&format!("http://{}/cgi-bin/get_blink_status.cgi", self.ip))
            .send_with_digest_auth(&self.username, &self.password)
            .await?;
        if resp.status().is_success() {
            let json = resp.json::<serde_json::Value>().await?;
            Ok(json["blink"].as_bool().ok_or(Error::ExpectedReturn)?)
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
        let resp = self.client.http_client
            .get(&format!("http://{}/cgi-bin/log.cgi", self.ip))
            .send_with_digest_auth(&self.username, &self.password)
            .await?;
        if resp.status().is_success() {
            Ok(resp.text().await?.lines().map(|s| s.to_string()).collect())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_mac(&self) -> Result<String, Error> {
        let sys_info = self.sys_info().await?;
        let sys_info = sys_info.as_ref().unwrap_or_else(|| unreachable!());

        Ok(sys_info.macaddr.clone())
    }

    async fn get_errors(&mut self) -> Result<Vec<String>, Error> {
        let log = self.get_logs().await?.join("\n");
        let mut errors = HashSet::new();
        for err in AntminerErrors.iter() {
            if let Some(msg) = err.get_msg(&log) {
                errors.insert(msg);
            }
        }
        Ok(errors.into_iter().collect())
    }

    async fn get_dns(&self) -> Result<String, Error> {
        let sys_info = self.sys_info().await?;
        let sys_info = sys_info.as_ref().unwrap_or_else(|| unreachable!());

        Ok(sys_info.dnsservers.clone())
    }
}
