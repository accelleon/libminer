use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct PoolConf {
    pub url: String,
    pub user: String,
    pub pass: String,
}

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
    pub pools: [PoolConf; 3],
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
}