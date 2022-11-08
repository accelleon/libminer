use serde::Deserialize;

use crate::miners::minerva::minera::{ActivePool, Devices, DeviceTotal, Ifconfig, Pool};

#[derive(Deserialize)]
pub struct RunningStats {
    pub start_time: usize,
    pub devices: Devices,
    pub totals: DeviceTotal,
    pub pool: ActivePool,
    pub pools: Vec<Pool>,
    //network_miners: JSONValue,
    pub minera_id: String,
    pub mac_addr: String,
    pub ifconfig: Ifconfig,
    pub miner: Option<String>,
    pub algo: String,
    pub sysload: [f64; 3],
    pub cron: Option<String>,
    pub sysuptime: String,
    pub temp: f64,
    //altcoin_rates: JSONValue,
    //avg: JSONValue,
    pub profits: Option<f64>,
    pub livestat: bool,
    pub timestamp: usize,
}

#[derive(Deserialize)]
pub struct NotRunningStats {
    pub notrunning: bool,
    pub network_miners: Vec<String>,
    pub minera_id: String,
    pub mac_addr: String,
    pub ifconfig: Ifconfig,
    pub miner: Option<String>,
    pub algo: String,
    pub sysload: [f64; 3],
    pub cron: Option<String>,
    pub sysuptime: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum StatsResp {
    Running(RunningStats),
    NotRunning(NotRunningStats),
}
