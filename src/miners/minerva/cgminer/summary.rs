use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SummaryData {
    #[serde(rename = "Accepted")]
    pub accepted: usize,
    #[serde(rename = "Best Share")]
    pub best_share: usize,
    #[serde(rename = "Device Hardware%")]
    pub device_hardware: f64,
    #[serde(rename = "Device Rejected%")]
    pub device_rejected: f64,
    #[serde(rename = "Difficulty Accepted")]
    pub difficulty_accepted: usize,
    #[serde(rename = "Difficulty Rejected")]
    pub difficulty_rejected: usize,
    #[serde(rename = "Difficulty Stale")]
    pub difficulty_stale: usize,
    #[serde(rename = "Discarded")]
    pub discarded: usize,
    #[serde(rename = "Elapsed")]
    pub elapsed: usize,
    #[serde(rename = "Found Blocks")]
    pub found_blocks: usize,
    #[serde(rename = "Get Failures")]
    pub get_failures: usize,
    #[serde(rename = "Getworks")]
    pub getworks: usize,
    #[serde(rename = "Hardware Errors")]
    pub hardware_errors: usize,
    #[serde(rename = "Last getwork")]
    pub last_getwork: usize,
    #[serde(rename = "Local Work")]
    pub local_work: usize,
    #[serde(rename = "MHS 15m")]
    pub mhs_15m: f64,
    #[serde(rename = "MHS 1m")]
    pub mhs_1m: f64,
    #[serde(rename = "MHS 5m")]
    pub mhs_5m: f64,
    #[serde(rename = "MHS 5s")]
    pub mhs_5s: f64,
    #[serde(rename = "MHS av")]
    pub mhs_av: f64,
    #[serde(rename = "Netid")]
    pub netid: String,
    #[serde(rename = "Network Blocks")]
    pub network_blocks: usize,
    #[serde(rename = "Pool Rejected%")]
    pub pool_rejected: f64,
    #[serde(rename = "Pool Stale%")]
    pub pool_stale: f64,
    #[serde(rename = "Rejected")]
    pub rejected: usize,
    #[serde(rename = "Remote Failures")]
    pub remote_failures: usize,
    #[serde(rename = "Stale")]
    pub stale: usize,
    #[serde(rename = "Total MH")]
    pub total_mh: f64,
    #[serde(rename = "Utility")]
    pub utility: f64,
    #[serde(rename = "Work Utility")]
    pub work_utility: f64,
}

#[derive(Deserialize, Debug)]
pub struct SummaryResp {
    pub code: usize,
    pub data: [SummaryData; 1],
    pub message: String,
}