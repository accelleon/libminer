use serde::Deserialize;

use crate::miners::antminer::cgi::{Status, CgiInfo};

#[derive(Deserialize, Debug)]
pub struct Chain {
    pub index: usize,
    pub freq_avg: usize,
    pub rate_ideal: f64,
    pub rate_real: f64,
    /// Number of detected ASIC chips
    pub asic_num: usize,
    /// String of detected ASIC chips, o for each successful with space between groups
    pub asic: String,
    pub temp_chip: Vec<usize>,
    pub temp_pcb: Vec<usize>,
    pub temp_pic: Vec<usize>,
    pub hw: u16,
    pub eeprom_loaded: bool,
    pub sn: String,
    pub hwp: f64,
    //pub tpl: Vec<Vec<usize>>,
}

#[derive(Deserialize, Debug)]
pub struct Stat {
    pub elapsed: usize,
    pub rate_5s: f64,
    pub rate_30m: f64,
    pub rate_avg: f64,
    pub rate_ideal: f64,
    pub rate_unit: String,
    pub chain_num: usize,
    pub fan_num: usize,
    pub fan: Vec<u32>,
    pub hwp_total: f64,
    #[serde(rename = "miner-mode")]
    pub miner_mode: usize,
    #[serde(rename = "freq-level")]
    pub freq_level: usize,
    pub chain: Vec<Chain>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct StatsResponse {
    pub info: CgiInfo,
    pub stats: Vec<Stat>,
    pub status: Status,
}