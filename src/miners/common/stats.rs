use serde::Deserialize;
use crate::miners::common::*;

#[derive(Debug, Deserialize)]
pub struct StatsShared {
    #[serde(rename = "STATS")]
    pub stats: usize,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Elapsed")]
    pub elapsed: usize,
    #[serde(rename = "Calls")]
    pub calls: usize,
    #[serde(rename = "Wait")]
    pub wait: f64,
    #[serde(rename = "Max")]
    pub max: f64,
    #[serde(rename = "Min")]
    pub min: f64,
}

#[derive(Deserialize, Debug)]
pub struct DevStats {
    #[serde(flatten)]
    pub shared: StatsShared,
    #[serde(rename = "Type")]
    pub type_: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PoolStats {
    #[serde(flatten)]
    pub shared: StatsShared,
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "Pool Calls")]
    pub pool_calls: usize,
    #[serde(rename = "Pool Attempts")]
    pub pool_attempts: usize,
    #[serde(rename = "Pool Wait")]
    pub pool_wait: f64,
    #[serde(rename = "Pool Max")]
    pub pool_max: f64,
    #[serde(rename = "Pool Min")]
    pub pool_min: f64,
    #[serde(rename = "Pool Av")]
    pub pool_av: f64,
    #[serde(rename = "Work Had Roll Time")]
    pub work_had_roll_time: bool,
    #[serde(rename = "Work Can Roll")]
    pub work_can_roll: bool,
    #[serde(rename = "Work Had Expire")]
    pub work_had_expire: bool,
    #[serde(rename = "Work Roll Time")]
    pub work_roll_time: usize,
    #[serde(rename = "Work Diff")]
    pub work_diff: f64,
    #[serde(rename = "Min Diff")]
    pub min_diff: f64,
    #[serde(rename = "Max Diff")]
    pub max_diff: f64,
    #[serde(rename = "Min Diff Count")]
    pub min_diff_count: usize,
    #[serde(rename = "Max Diff Count")]
    pub max_diff_count: usize,
    #[serde(rename = "Times Sent")]
    pub times_sent: usize,
    #[serde(rename = "Bytes Sent")]
    pub bytes_sent: usize,
    #[serde(rename = "Times Recv")]
    pub times_recv: usize,
    #[serde(rename = "Bytes Recv")]
    pub bytes_recv: usize,
    #[serde(rename = "Net Bytes Sent")]
    pub net_bytes_sent: usize,
    #[serde(rename = "Net Bytes Recv")]
    pub net_bytes_recv: usize,
}

/// Antminer stats section including model and version
#[derive(Deserialize, Debug)]
pub struct AmVersion {
    #[serde(rename = "BMMiner")]
    pub bmminer: String,
    #[serde(rename = "Miner")]
    pub miner: String,
    #[serde(rename = "CompileTime")]
    pub compile_time: String,
    #[serde(rename = "Type")]
    pub type_: String,
}

/// Antminer stats section including current device stats
#[derive(Deserialize, Debug)]
pub struct AmStats {
    #[serde(flatten)]
    pub shared: StatsShared,
    #[serde(rename = "GHS 5s")]
    pub ghs_5s: f64,
    #[serde(rename = "GHS av")]
    pub ghs_av: f64,
    #[serde(rename = "rate_30m")]
    pub rate_30m: f64,
    #[serde(rename = "Mode")]
    pub mode: usize,
    pub miner_count: usize,
    pub frequency: usize,
    pub fan_num: usize,
    pub fan1: usize,
    pub fan2: usize,
    pub fan3: usize,
    pub fan4: usize,
    pub temp_num: usize,
    pub temp1: usize,
    pub temp2: usize,
    pub temp2_1: usize,
    pub temp2_2: usize,
    pub temp2_3: usize,
    pub temp3: usize,
    pub temp_pcb1: String,
    pub temp_pcb2: String,
    pub temp_pcb3: String,
    pub temp_pcb4: String,
    pub temp_chip1: String,
    pub temp_chip2: String,
    pub temp_chip3: String,
    pub temp_chip4: String,
    pub temp_pic1: String,
    pub temp_pic2: String,
    pub temp_pic3: String,
    pub temp_pic4: String,
    pub total_rateideal: f64,
    pub rate_unit: String,
    pub total_freqavg: usize,
    pub total_acn: usize,
    #[serde(rename = "total rate")]
    pub total_rate: f64,
    pub temp_max: usize,
    pub no_matching_work: usize,
    pub chain_acn1: usize,
    pub chain_acn2: usize,
    pub chain_acn3: usize,
    pub chain_acn4: usize,
    pub chain_acs1: Option<String>,
    pub chain_acs2: Option<String>,
    pub chain_acs3: Option<String>,
    pub chain_acs4: Option<String>,
    pub chain_hw1: usize,
    pub chain_hw2: usize,
    pub chain_hw3: usize,
    pub chain_hw4: usize,
    pub chain_rate1: String,
    pub chain_rate2: String,
    pub chain_rate3: String,
    pub chain_rate4: String,
    pub freq1: usize,
    pub freq2: usize,
    pub freq3: usize,
    pub freq4: usize,
    pub miner_version: String,
    pub miner_id: String,
}

/// Avalon stats section
/// wtf Avalon?
#[derive(Deserialize, Debug)]
pub struct AvaStats {
    #[serde(flatten)]
    pub shared: StatsShared,
    #[serde(rename = "MM ID0")]
    pub mm_id0: String,
}

/// Enum of a variety of stat sections that can be returned
/// from {"command": "stats"}
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Stats {
    Pool(PoolStats), // Ensure PoolStats is attempted first
    AvaStats(AvaStats),
    Dev(DevStats),
    AmVersion(AmVersion),
    AmStats(AmStats),
}

#[derive(Deserialize, Debug)]
pub struct StatsResp {
    #[serde(rename = "STATUS")]
    pub status: [Status; 1],
    #[serde(rename = "STATS")]
    pub stats: Option<Vec<Stats>>,
}
