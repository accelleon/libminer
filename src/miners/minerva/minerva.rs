use std::collections::HashMap;

use async_trait::async_trait;
use reqwest::multipart::Form;
use serde_json::json;
use tracing::{warn, error};
use std::collections::HashSet;
use scraper::{Html, Selector};
use tokio::sync::{Mutex, MutexGuard};
use crate::Client;
use crate::miner::{Miner, Pool};
use crate::error::Error;
use crate::miners::minerva::{cgminer, minera};
use crate::miners::minerva::error::{MinerVaErrors, MineraErrors};

/// 4 fan Minervas use this interface
pub struct Minera {
    ip: String,
    port: u16,
    client: Client,

    stats: Mutex<Option<minera::StatsResp>>,
}

impl Minera {
    async fn get_stats(&self) -> Result<MutexGuard<Option<minera::StatsResp>>, Error> {
        let mut stats = self.stats.lock().await;
        if stats.is_none() {
            let resp = self.client.http_client
                .get(&format!("http://{}/index.php/app/stats", self.ip))
                .send()
                .await?;
            if resp.status().is_success() {
                let stat: minera::StatsResp = resp.json().await?;
                *stats = Some(stat);
            } else {
                return Err(Error::HttpRequestFailed);
            }
        }
        Ok(stats)
    }

    async fn invalidate_stats(&self) {
        let _ = self.stats.lock().await.take();
    }

    /// Returns the number of hashboards detected and the number online
    async fn get_board_count(&self) -> Result<u8, Error> {
        let stat = self.get_stats().await?;
        let stat = stat.as_ref().unwrap();
        if let minera::StatsResp::Running(stat) = stat {
            if stat.devices.board_4.is_some() {
                Ok(4)
            } else if stat.devices.board_3.is_some() {
                Ok(3)
            } else if stat.devices.board_2.is_some() {
                Ok(2)
            } else if stat.devices.board_1.is_some() {
                Ok(1)
            } else {
                Ok(0)
            }
        } else {
            Err(Error::InvalidResponse)
        }
    }
}

#[async_trait]
impl Miner for Minera {
    fn new(client: Client, ip: String, port: u16) -> Self {
        Minera {
            ip,
            port,
            client,
            stats: Mutex::new(None),
        }
    }

    fn get_type(&self) -> &'static str {
        "MinerVa"
    }

    async fn get_model(&self) -> Result<String, Error> {
        //The below doesn't respond when the miner is not running
        // let resp = self.client.send_recv(&self.ip, self.port, &json!({"command":"devdetails"})).await?;
        // let js = serde_json::from_str::<common::DevDetailsResp>(&resp)?;
        // Ok(js.devdetails.get(0).unwrap().model.clone())
        Ok("MV7 4Fan".to_string())
    }

    async fn auth(&mut self, _username: &str, password: &str) -> Result<(), Error> {
        let mut form = HashMap::new();
        form.insert("password", password);
        let resp = self.client.http_client
            .post(&format!("http://{}/index.php/app/login", self.ip))
            .form(&form)
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn reboot(&mut self) -> Result<(), Error> {
        //TODO: This always times out as the API reboots before responding
        let resp = self.client.http_client
            .post(&format!("http://{}/index.php/app/reboot", self.ip))
            .query(&[("confirm", "1")])
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_hashrate(&self) -> Result<f64, Error> {
        let stat = self.get_stats().await?;
        let stat = stat.as_ref().unwrap();
        if let minera::StatsResp::Running(stat) = stat {
            // Convert to TH/S
            Ok(stat.totals.hashrate / 1000000000000.0)
        } else {
            Ok(0.0)
        }
    }

    async fn get_power(&self) -> Result<f64, Error> {
        // Guess at power consumption
        // There are 3 models with efficiencies ranging from 31 - 39 J/TH
        // Assume the middle of the road 35 J/TH
        Ok(self.get_hashrate().await? * 35.0)
    }

    async fn get_efficiency(&self) -> Result<f64, Error> {
        Ok(35.0)
    }

    async fn get_nameplate_rate(&self) -> Result<f64, Error> {
        // Minerva doesn't report a nameplate rate, so we have to guess
        // There are 3 models with hashrates varying from 75 to 105 TH/s
        // Assume the middle of the road 90 TH/s
        Ok(90.0)
    }

    async fn get_temperature(&self) -> Result<f64, Error> {
        let stat = self.get_stats().await?;
        let stat = stat.as_ref().unwrap();
        if let minera::StatsResp::Running(stat) = stat {
            // Convert to TH/S
            Ok(stat.temp)
        } else {
            Ok(0.0)
        }
    }

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error> {
        Ok(vec![])
    }

    async fn get_pools(&self) -> Result<Vec<Pool>, Error> {
        // To get pools for miners not running we need to parse raw html .-.
        // We can look for poolSortable as the container, each pool is a new pool-group
        let pools_selector = Selector::parse(".poolSortable").unwrap();
        let pool_group_selector = Selector::parse(".pool-group").unwrap();
        let pool_url_selector = Selector::parse(r#"input[name="pool_url[]"]"#).unwrap();
        let pool_user_selector = Selector::parse(r#"input[name="pool_username[]"]"#).unwrap();
        let pool_pass_selector = Selector::parse(r#"input[name="pool_password[]"]"#).unwrap();
        let resp = self.client.http_client
            .get(&format!("http://{}/index.php/app/settings", self.ip))
            .send()
            .await?;
        let document = Html::parse_document(resp.text().await?.as_str());
        if let Some(pools) = document.select(&pools_selector).next() {
            let mut pool_list = vec![];
            for pool in pools.select(&pool_group_selector) {
                let url = pool.select(&pool_url_selector).next().unwrap().value().attr("value").unwrap().to_string();
                let user = pool.select(&pool_user_selector).next().unwrap().value().attr("value").unwrap().to_string();
                let pass = pool.select(&pool_pass_selector).next().unwrap().value().attr("value").unwrap().to_string();
                pool_list.push(Pool {
                    url,
                    username: user,
                    password: if pass.is_empty() {None} else {Some(pass)},
                });
            }
            Ok(pool_list)
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error> {
        let mut form = Form::new()
            .text("save_miner_pools", "1");
        
        for pool in pools {
            form = form
                .text("pool_url[]", pool.url.clone())
                .text("pool_username[]", pool.username.clone())
                .text("pool_password[]", if let Some(ref password) = pool.password {
                    password.clone()
                } else {
                    "".to_string()
                });
        }

        let resp = self.client.http_client
            .post(&format!("http://{}/index.php/app/settings", self.ip))
            .multipart(form)
            .send()
            .await?;
        if resp.status().is_success() {
            self.invalidate_stats().await;
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_sleep(&self) -> Result<bool, Error> {
        Err(Error::NotSupported)
    }

    async fn set_sleep(&mut self, _sleep: bool) -> Result<(), Error> {
        return Err(Error::NotSupported);
    }

    async fn get_blink(&self) -> Result<bool, Error> {
        Err(Error::NotSupported)
    }

    async fn set_blink(&mut self, blink: bool) -> Result<(), Error> {
        Err(Error::NotSupported)
    }

    async fn get_logs(&mut self) -> Result<Vec<String>, Error> {
        // /index.php/app/varLog
        // This returns everything, we're gonna want to subscript it
        let resp = self.client.http_client
            .get(&format!("http://{}/index.php/app/varLog", self.ip))
            .send()
            .await?;
        if resp.status().is_success() {
            let text = resp.text().await?;
            Ok(text.lines().map(|s| s.to_string()).collect())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_mac(&self) -> Result<String, Error> {
        let stat = self.get_stats().await?;
        let stat = stat.as_ref().unwrap();
        match stat {
            minera::StatsResp::Running(stat) => Ok(stat.mac_addr.clone()),
            minera::StatsResp::NotRunning(stat) => Ok(stat.mac_addr.clone()),
        }
    }

    async fn get_errors(&mut self) -> Result<Vec<String>, Error> {
        // We're going to only keep the last 300 lines
        // as this returns logs from before jesus was born
        let log = self.get_logs().await?
            .iter()
            .rev()
            .take(300)
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let mut errors = HashSet::new();
        for err in MineraErrors.iter() {
            if let Some(msg) = err.get_msg(&log) {
                errors.insert(msg);
            }
        }
        Ok(errors.into_iter().collect())
    }

    async fn get_dns(&self) -> Result<String, Error> {
        let stat = self.get_stats().await?;
        let stat = stat.as_ref().unwrap();
        match stat {
            minera::StatsResp::Running(stat) => Ok(stat.ifconfig.dns.clone()),
            minera::StatsResp::NotRunning(stat) => Ok(stat.ifconfig.dns.clone()),
        }
    }
}

/// 2 fan Minervas use this interface
pub struct Minerva {
    ip: String,
    port: u16,
    client: Client,
    token: String,
}

impl Minerva {
    /// Returns the number of hashboards detected
    async fn get_board_count(&self) -> Result<u8, Error> {
        let resp = self.client.http_client
            .get(&format!("https://{}/api/v1/systemInfo/hashBoards", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        if resp.status().is_success() {
            let resp = resp.json::<cgminer::HashBoardsResp>().await.unwrap();
            let boards = resp.data.ok_or(Error::ApiCallFailed(resp.message))?;
            Ok(boards.len() as u8)
        } else {
            Err(Error::HttpRequestFailed)
        }
    }
}

#[async_trait]
impl Miner for Minerva {
    fn new(client: Client, ip: String, port: u16) -> Self {
        Minerva {
            ip,
            port,
            client,
            token: "".to_string(),
        }
    }

    fn get_type(&self) -> &'static str {
        "MinerVa"
    }

    async fn get_model(&self) -> Result<String, Error> {
        // let resp = self.client.send_recv(&self.ip, self.port, &json!({"command":"devdetails"})).await?;
        // let js = serde_json::from_str::<common::DevDetailsResp>(&resp)?;
        // Ok(js.devdetails.get(0).unwrap().model.clone())
        Ok("MV7".into())
    }

    async fn auth(&mut self, username: &str, password: &str) -> Result<(), Error> {
        let resp = self.client.http_client
            .post(&format!("https://{}/api/v1/auth/login", self.ip))
            .json(&json!({
                "username": username,
                "password": password,
            }))
            .send()
            .await?;
        if resp.status().is_success() {
            let text = resp.text().await?;
            if let Ok(js) = serde_json::from_str::<cgminer::AuthResp>(&text) {
                self.token = js.data.access_token.clone();
                Ok(())
            } else if let Ok(_) = serde_json::from_str::<cgminer::ApiResp>(&text) {
                //TODO: Check returned status code and return appropriate error
                Err(Error::Unauthorized)
            } else {
                Err(Error::UnknownMinerType)
            }
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn reboot(&mut self) -> Result<(), Error> {
        //TODO: This always times out as the API reboots before responding
        let resp = self.client.http_client
            .post(&format!("https://{}:/api/v1/cgminer/reboot", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await;
        Ok(())
    }

    async fn get_hashrate(&self) -> Result<f64, Error> {
        let resp = self.client.http_client
            .get(&format!("https://{}/api/v1/cgminer/summary", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        if resp.status().is_success() {
            let text = resp.text().await?;
            if let Ok(summary) = serde_json::from_str::<cgminer::SummaryResp>(&text) {
                // Convert to TH/s
                Ok(summary.data[0].mhs_5s / 1000000.0)
            } else if let Ok(status) = serde_json::from_str::<cgminer::ApiResp>(&text) {
                // The miners up but didn't give us a great response, so just return 0
                Ok(0.0)
            } else {
                Err(Error::ApiCallFailed("Unknown error".to_string()))
            }
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_power(&self) -> Result<f64, Error> {
        // Guess at power consumption
        // There are 3 models with efficiencies ranging from 31 - 39 J/TH
        // Assume the middle of the road 35 J/TH
        Ok(self.get_hashrate().await? * 35.0)
    }

    async fn get_efficiency(&self) -> Result<f64, Error> {
        Ok(35.0)
    }

    async fn get_nameplate_rate(&self) -> Result<f64, Error> {
        // Minerva doesn't report a nameplate rate, so we have to guess
        // There are 3 models with hashrates varying from 75 to 105 TH/s
        // Assume the middle of the road 90 TH/s
        Ok(90.0)
    }

    async fn get_temperature(&self) -> Result<f64, Error> {
        let resp = self.client.http_client
            .get(&format!("https://{}/api/v1/systemInfo/tempAndSpeed", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        if resp.status().is_success() {
            let temp = resp.json::<cgminer::TempAndSpeedResp>().await?;
            Ok(temp.data.temperature)
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error> {
        let resp = self.client.http_client
            .get(&format!("https://{}/api/v1/systemInfo/tempAndSpeed", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        if resp.status().is_success() {
            let temp = resp.json::<cgminer::TempAndSpeedResp>().await?;
            Ok(vec![temp.data.fan_speed1, temp.data.fan_speed2])
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_pools(&self) -> Result<Vec<Pool>, Error> {
        let resp = self.client.http_client
            .get(&format!("https://{}/api/v1/cgminer/poolsInSetting", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        if resp.status().is_success() {
            let pools = resp.json::<cgminer::GetPoolsResp>().await?;
            let mut ret = Vec::new();
            ret.push(Pool {
                url: pools.data.pool1url,
                username: pools.data.pool1user,
                password: None,
            });
            ret.push(Pool {
                url: pools.data.pool2url,
                username: pools.data.pool2user,
                password: None,
            });
            ret.push(Pool {
                url: pools.data.pool3url,
                username: pools.data.pool3user,
                password: None,
            });
            Ok(ret)
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error> {
        let resp = self.client.http_client
            .post(&format!("https://{}/api/v1/cgminer/changePool", self.ip))
            .bearer_auth(&self.token)
            .json(&cgminer::SetPoolRequest {
                pool1url: &pools[0].url,
                pool1user: &pools[0].username,
                pool1pwd: if let Some(pwd) = &pools[0].password {&pwd} else {""},
                pool2url: &pools[1].url,
                pool2user: &pools[1].username,
                pool2pwd: if let Some(pwd) = &pools[1].password {&pwd} else {""},
                pool3url: &pools[2].url,
                pool3user: &pools[2].username,
                pool3pwd: if let Some(pwd) = &pools[2].password {&pwd} else {""},
            })
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_sleep(&self) -> Result<bool, Error> {
        let resp1 = self.client.http_client
            .get(&format!("https://{}/api/v1/cgminer/workMode", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        if resp1.status().is_success() {
            let js = resp1.json::<serde_json::Value>().await?;
            if let Some(mask) = js["data"]["mask"].as_str() {
                Ok(mask == "0x0")
            } else {
                Err(Error::ExpectedReturn)
            }
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn set_sleep(&mut self, sleep: bool) -> Result<(), Error> {
        let resp1 = self.client.http_client
            .get(&format!("https://{}/api/v1/cgminer/workMode", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        //println!("{}", resp1.text().await.unwrap());
        let js = resp1.json::<serde_json::Value>().await?;
        let mut hash = js.as_object().unwrap().clone();
        let data = hash.get_mut("data").unwrap();
        //data["mask"] = serde_json::Value::from(if sleep { "0x0" } else { "0xf" });
        let mut default = serde_json::Map::new();
        let data = data.as_object_mut().unwrap_or(&mut default);
        data.remove("mask");
        data.insert("mask".to_string(), serde_json::Value::from(if sleep { "0x0" } else { "0xf" }));
        //println!("{:?}", data);
        let resp = self.client.http_client
            .post(&format!("https://{}/api/v1/cgminer/setWorkMode", self.ip))
            .bearer_auth(&self.token)
            .json(&data)
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_blink(&self) -> Result<bool, Error> {
        let resp = self.client.http_client
            .get(&format!("https://{}/api/v1/systemInfo/redLedStatus", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        if resp.status().is_success() {
            let led = resp.json::<cgminer::LedResp>().await?;
            Ok(led.data.status == "1")
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn set_blink(&mut self, blink: bool) -> Result<(), Error> {
        let status = cgminer::LedStatus {
            status: (if blink { "1" } else { "0" }).to_string(),
        };
        let resp = self.client.http_client
            .post(&format!("https://{}/api/v1/systemInfo/setRedLedStatus", self.ip))
            .bearer_auth(&self.token)
            .json(&status)
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_logs(&mut self) -> Result<Vec<String>, Error> {
        let resp = self.client.http_client
            .get(&format!("https://{}/api/v1/cgminer/log", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        if resp.status().is_success() {
            let logs = resp.json::<cgminer::LogResp>().await?;
            Ok(logs.data)
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_mac(&self) -> Result<String, Error> {
        let resp = self.client.http_client
            .get(&format!("https://{}/api/v1/systemInfo/network", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        if resp.status().is_success() {
            let network = resp.json::<cgminer::NetworkResponse>().await?;
            Ok(network.data.hardwareAddress)
        } else {
            Err(Error::HttpRequestFailed)
        }
    }

    async fn get_errors(&mut self) -> Result<Vec<String>, Error> {
        let log = self.get_logs().await?.join("\n");
        let mut errors = HashSet::new();
        for err in MinerVaErrors.iter() {
            if let Some(msg) = err.get_msg(&log) {
                errors.insert(msg);
            }
        }
        Ok(errors.into_iter().collect())
    }

    async fn get_dns(&self) -> Result<String, Error> {
        let resp = self.client.http_client
            .get(&format!("https://{}/api/v1/systemInfo/network", self.ip))
            .bearer_auth(&self.token)
            .send()
            .await?;
        if resp.status().is_success() {
            let network = resp.json::<cgminer::NetworkResponse>().await?;
            Ok(network.data.dns.clone())
        } else {
            Err(Error::HttpRequestFailed)
        }
    }
}
