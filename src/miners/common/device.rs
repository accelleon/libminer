use serde::Deserialize;
use crate::miners::common::*;

#[derive(Deserialize, Debug)]
pub struct Device {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Enabled")]
    pub enabled: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Temperature")]
    pub temperature: f64,
    #[serde(rename = "MHS av")]
    pub mhs_av: f64,
    #[serde(rename = "MHS 5s")]
    pub mhs_5s: f64,
    #[serde(rename = "MHS 1m")]
    pub mhs_1m: f64,
    #[serde(rename = "MHS 5m")]
    pub mhs_5m: f64,
    #[serde(rename = "MHS 15m")]
    pub mhs_15m: f64,
    #[serde(rename = "Accepted")]
    pub accepted: usize,
    #[serde(rename = "Rejected")]
    pub rejected: usize,
    #[serde(rename = "Hardware Errors")]
    pub hw_errors: usize,
    #[serde(rename = "Utility")]
    pub utility: f64,
    #[serde(rename = "Last Share Pool")]
    pub last_share_pool: usize,
    #[serde(rename = "Last Share Time")]
    pub last_share_time: usize,
    #[serde(rename = "Total MH")]
    pub total_mh: f64,
    #[serde(rename = "Diff1 Work")]
    pub diff1_work: usize,
    #[serde(rename = "Difficulty Accepted")]
    pub difficulty_accepted: f64,
    #[serde(rename = "Difficulty Rejected")]
    pub difficulty_rejected: f64,
    #[serde(rename = "Last Share Difficulty")]
    pub last_share_difficulty: f64,
    #[serde(rename = "Last Valid Work")]
    pub last_valid_work: usize,
    #[serde(rename = "Device Hardware%")]
    pub device_hardware_per: f64,
    #[serde(rename = "Device Rejected%")]
    pub device_rejected_per: f64,
    #[serde(rename = "Device Elapsed")]
    pub device_elapsed: usize,
}

#[derive(Deserialize, Debug)]
pub struct DevsResp {
    #[serde(rename = "STATUS")]
    pub status: Vec<Status>,
    #[serde(rename = "DEVS")]
    pub devs: Vec<Device>,
}

#[derive(Deserialize, Debug)]
pub struct DevDetails {
    #[serde(rename = "DEVDETAILS")]
    pub devdetails: usize,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ID")]
    pub id: usize,
    #[serde(rename = "Driver")]
    pub driver: String,
    #[serde(rename = "Kernel")]
    pub kernel: String,
    #[serde(rename = "Model")]
    pub model: String,
    #[serde(rename = "Device Path")]
    pub device_path: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct DevDetailsResp {
    #[serde(rename = "STATUS")]
    pub status: Vec<Status>,
    #[serde(rename = "DEVDETAILS")]
    pub devdetails: Vec<DevDetails>,
}