use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct SetPoolRequest <'a> {
    /// /api/v1/cgminer/changePool
    pub pool0url: &'a str,
    pub pool0user: &'a str,
    pub pool0pwd: &'a str,
    pub pool1url: &'a str,
    pub pool1user: &'a str,
    pub pool1pwd: &'a str,
    pub pool2url: &'a str,
    pub pool2user: &'a str,
    pub pool2pwd: &'a str,
}

#[derive(Deserialize, Debug)]
pub struct GetPools {
    pub pool1url: String,
    pub pool1user: String,
    pub pool2url: String,
    pub pool2user: String,
    pub pool3url: String,
    pub pool3user: String,
}

#[derive(Deserialize, Debug)]
pub struct PoolDesc {
    #[serde(rename = "Accepted")]
    pub accepted: usize,
    #[serde(rename = "Bad Work")]
    pub bad_work: usize,
    #[serde(rename = "Best Share")]
    pub best_share: usize,
    #[serde(rename = "Current Block Height")]
    pub current_block_height: usize,
    #[serde(rename = "Current Block Version")]
    pub current_block_version: usize,
    #[serde(rename = "Diff1 Shares")]
    pub diff1_shares: usize,
    #[serde(rename = "Difficulty Accepted")]
    pub difficulty_accepted: usize,
    #[serde(rename = "Difficulty Rejected")]
    pub difficulty_rejected: usize,
    #[serde(rename = "Difficulty Stale")]
    pub difficulty_stale: usize,
    #[serde(rename = "Discarded")]
    pub discarded: usize,
    #[serde(rename = "Get Failures")]
    pub get_failures: usize,
    #[serde(rename = "Getworks")]
    pub getworks: usize,
    #[serde(rename = "Has GBT")]
    pub has_gbt: bool,
    #[serde(rename = "Has Stratum")]
    pub has_stratum: bool,
    #[serde(rename = "Has Vmask")]
    pub has_vmask: bool,
    #[serde(rename = "Last Share Difficulty")]
    pub last_share_difficulty: usize,
    #[serde(rename = "Last Share Time")]
    pub last_share_time: usize,
    #[serde(rename = "Long Poll")]
    pub long_poll: String,
    #[serde(rename = "POOL")]
    pub pool: usize,
    #[serde(rename = "Pool Rejected%")]
    pub pool_rejected_percent: f64,
    #[serde(rename = "Pool Stale%")]
    pub pool_stale_percent: f64,
    #[serde(rename = "Priority")]
    pub priority: usize,
    #[serde(rename = "Proxy")]
    pub proxy: String,
    #[serde(rename = "Proxy Type")]
    pub proxy_type: String,
    #[serde(rename = "Quota")]
    pub quota: usize,
    #[serde(rename = "Rejected")]
    pub rejected: usize,
    #[serde(rename = "Remote Failures")]
    pub remote_failures: usize,
    #[serde(rename = "Stale")]
    pub stale: usize,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Stratum Active")]
    pub stratum_active: bool,
    #[serde(rename = "Stratum Difficulty")]
    pub stratum_difficulty: usize,
    #[serde(rename = "Stratum URL")]
    pub stratum_url: String,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "User")]
    pub user: String,
    #[serde(rename = "Work Difficulty")]
    pub work_difficulty: usize,
    #[serde(rename = "Works")]
    pub works: usize,
}

#[derive(Deserialize, Debug)]
pub struct GetPoolsStatsResp {
    pub code: usize,
    pub data: [PoolDesc; 3],
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct GetPoolsResp {
    pub code: usize,
    pub data: GetPools,
    pub message: String,
}