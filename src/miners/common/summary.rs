use serde::Deserialize;

use crate::miners::common::*;

#[derive(Deserialize, Debug)]
pub struct Summary {
    #[serde(rename = "Elapsed")]
    pub elapsed: usize,
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
    #[serde(rename = "Found Blocks")]
    pub found_blocks: usize,
    #[serde(rename = "Getworks")]
    pub getworks: usize,
    #[serde(rename = "Accepted")]
    pub accepted: usize,
    #[serde(rename = "Rejected")]
    pub rejected: usize,
    #[serde(rename = "Hardware Errors")]
    pub hw_errors: usize,
    #[serde(rename = "Utility")]
    pub utility: f64,
    #[serde(rename = "Discarded")]
    pub discarded: usize,
    #[serde(rename = "Stale")]
    pub stale: usize,
    #[serde(rename = "Get Failures")]
    pub get_failures: usize,
    #[serde(rename = "Local Work")]
    pub local_work: usize,
    #[serde(rename = "Remote Failures")]
    pub remote_failures: usize,
    #[serde(rename = "Network Blocks")]
    pub network_blocks: usize,
    #[serde(rename = "Total MH")]
    pub total_mh: f64,
    #[serde(rename = "Work Utility")]
    pub work_utility: f64,
    #[serde(rename = "Difficulty Accepted")]
    pub difficulty_accepted: f64,
    #[serde(rename = "Difficulty Rejected")]
    pub difficulty_rejected: f64,
    #[serde(rename = "Difficulty Stale")]
    pub difficulty_stale: f64,
    #[serde(rename = "Best Share")]
    pub best_share: usize,
    #[serde(rename = "Device Hardware%")]
    pub device_hardware_per: f64,
    #[serde(rename = "Device Rejected%")]
    pub device_rejected_per: f64,
    #[serde(rename = "Pool Rejected%")]
    pub pool_rejected_per: f64,
    #[serde(rename = "Pool Stale%")]
    pub pool_stale_per: f64,
    #[serde(rename = "Last getwork")]
    pub last_getwork: usize,
    #[serde(rename = "Netid")]
    pub netid: String,
}

#[derive(Deserialize, Debug)]
pub struct SummaryResp {
    #[serde(rename = "STATUS")]
    pub status: Vec<Status>,
    #[serde(rename = "SUMMARY")]
    pub summary: Vec<Summary>,
}