use serde::Deserialize;

use crate::miners::common::*;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Bool {
    U8(u8),
    BOOL(bool),
}

#[derive(Deserialize, Debug)]
pub struct PoolDesc {
    #[serde(rename = "POOL")]
    pub pool: usize,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Priority")]
    pub priority: usize,
    #[serde(rename = "Quota")]
    pub quota: usize,
    #[serde(rename = "Long Poll")]
    pub long_poll: String,
    #[serde(rename = "Getworks")]
    pub getworks: usize,
    #[serde(rename = "Accepted")]
    pub accepted: usize,
    #[serde(rename = "Rejected")]
    pub rejected: usize,
    #[serde(rename = "Works")]
    pub works: usize,
    #[serde(rename = "Discarded")]
    pub discarded: usize,
    #[serde(rename = "Stale")]
    pub stale: usize,
    #[serde(rename = "Get Failures")]
    pub get_failures: usize,
    #[serde(rename = "Remote Failures")]
    pub remote_failures: usize,
    #[serde(rename = "User")]
    pub user: String,
    #[serde(rename = "Last Share Time")]
    pub last_share_time: usize,
    #[serde(rename = "Diff1 Shares")]
    pub diff1_shares: usize,
    #[serde(rename = "Proxy Type")]
    pub proxy_type: String,
    #[serde(rename = "Proxy")]
    pub proxy: String,
    #[serde(rename = "Difficulty Accepted")]
    pub difficulty_accepted: f64,
    #[serde(rename = "Difficulty Rejected")]
    pub difficulty_rejected: f64,
    #[serde(rename = "Difficulty Stale")]
    pub difficulty_stale: f64,
    #[serde(rename = "Last Share Difficulty")]
    pub last_share_difficulty: f64,
    #[serde(rename = "Work Difficulty")]
    pub work_difficulty: f64,
    #[serde(rename = "Has Stratum")]
    pub has_stratum: Bool,
    #[serde(rename = "Stratum Active")]
    pub stratum_active: bool,
    #[serde(rename = "Stratum URL")]
    pub stratum_url: String,
    #[serde(rename = "Stratum Difficulty")]
    pub stratum_difficulty: f64,
    #[serde(rename = "Has Vmask")]
    pub has_vmask: Option<bool>,
    #[serde(rename = "Has GBT")]
    pub has_gbt: bool,
    #[serde(rename = "Best Share")]
    pub best_share: usize,
    #[serde(rename = "Pool Rejected%")]
    pub pool_rejected_percent: f64,
    #[serde(rename = "Pool Stale%")]
    pub pool_stale_percent: f64,
    #[serde(rename = "Bad Work")]
    pub bad_work: usize,
    #[serde(rename = "Current Block Height")]
    pub current_block_height: usize,
    #[serde(rename = "Current Block Version")]
    pub current_block_version: usize,
}

#[derive(Deserialize, Debug)]
pub struct PoolsResp {
    #[serde(rename = "STATUS")]
    pub status: Vec<Status>,
    #[serde(rename = "POOLS")]
    pub pools: Vec<PoolDesc>,
}