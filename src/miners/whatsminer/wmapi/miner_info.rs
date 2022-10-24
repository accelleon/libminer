use serde::Deserialize;

use crate::miners::common;

#[derive(Debug, Deserialize)]
pub struct MinerInfo {
    pub ip: String,
    pub proto: String,
    pub netmask: String,
    pub dns: String,
    pub mac: String,
    pub ledstat: String,
    pub gateway: String,
}

#[derive(Debug, Deserialize)]
pub struct MinerInfoResponse {
    #[serde(rename = "STATUS")]
    pub status: common::StatusCode,
    #[serde(rename = "When")]
    pub when: usize,
    #[serde(rename = "Code")]
    pub code: usize,
    #[serde(rename = "Msg")]
    pub msg: MinerInfo,
    #[serde(rename = "Description")]
    pub description: String,
}
