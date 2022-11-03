use serde::{Deserialize, Serialize};
use crate::Pool;

#[derive(Deserialize, Debug)]
pub struct GetConfResponse {
    #[serde(rename = "api-allow")]
    pub api_allow: String,
    #[serde(rename = "api-groups")]
    pub api_groups: String,
    #[serde(rename = "api-listen")]
    pub api_listen: bool,
    #[serde(rename = "api-network")]
    pub api_network: bool,
    #[serde(rename = "bitmain-ccdelay")]
    pub bitmain_ccdelay: String,
    #[serde(rename = "bitmain-fan-ctrl")]
    pub bitmain_fan_ctrl: bool,
    #[serde(rename = "bitmain-fan-pwm")]
    pub bitmain_fan_pwm: String,
    #[serde(rename = "bitmain-freq")]
    pub bitmain_freq: String,
    #[serde(rename = "bitmain-freq-level")]
    pub bitmain_freq_level: String,
    #[serde(rename = "bitmain-pwth")]
    pub bitmain_pwth: String,
    #[serde(rename = "bitmain-use-vil")]
    pub bitmain_use_vil: bool,
    #[serde(rename = "bitmain-voltage")]
    pub bitmain_voltage: String,
    /// "0" is normal, "1" is sleep
    #[serde(rename = "bitmain-work-mode")]
    pub bitmain_work_mode: String,
    pub pools: Vec<Pool>,
}

#[derive(Serialize, Debug)]
pub struct SetConf {
    #[serde(rename = "bitmain-fan-ctrl")]
    pub bitmain_fan_ctrl: bool,
    #[serde(rename = "bitmain-fan-pwm")]
    pub bitmain_fan_pwm: String,
    #[serde(rename = "freq-level")]
    pub freq_level: String,
    /// 0 is normal, 1 is sleep
    #[serde(rename = "miner-mode")]
    pub miner_mode: u8,
    pub pools: Vec<Pool>,
}

impl From<GetConfResponse> for SetConf {
    fn from(conf: GetConfResponse) -> Self {
        SetConf {
            bitmain_fan_ctrl: conf.bitmain_fan_ctrl,
            bitmain_fan_pwm: conf.bitmain_fan_pwm,
            freq_level: conf.bitmain_freq_level,
            // Antminers sometimes have this empty, default to 0 (normal)
            miner_mode: conf.bitmain_work_mode.parse().unwrap_or(0),
            pools: conf.pools,
        }
    }
}
