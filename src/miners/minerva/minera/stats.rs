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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_de() {
        let json = r#"{"start_time":1677253775,"devices":{"Board-1":{"temperature":94.61,"frequency":460,"accepted":26372,"rejected":145,"hw_errors":104860,"shares":448152061546,"hashrate":26173562130000,"last_share":1051774,"serial":false},"Board-2":{"temperature":82.2,"frequency":460,"accepted":25731,"rejected":119,"hw_errors":423466,"shares":436104678326,"hashrate":25469863670000,"last_share":1051774,"serial":false},"Board-3":{"temperature":96.71,"frequency":460,"accepted":15767,"rejected":49,"hw_errors":35748,"shares":290140573229,"hashrate":16948729739999.998,"last_share":1051774,"serial":false}},"totals":{"temperature":91.17,"frequency":460,"accepted":67870,"rejected":313,"hw_errors":564074,"shares":1174397313101,"hashrate":68592155540000,"last_share":1678305549},"pool":{"hashrate":68592166515920,"url":"stratum+tcp:\/\/btc.foundryusapool.com:3333","user":"pcminervas.3.4x97","alive":1},"pools":[{"priority":0,"url":"stratum+tcp:\/\/btc.foundryusapool.com:3333","active":true,"user":"pcminervas.3.4x97","pass":false,"stats":[{"start_time":false,"accepted":64878,"rejected":251,"shares":79127901,"stop_time":false,"stats_id":1}],"stats_id":1,"alive":1},{"priority":1,"url":"stratum+tcp:\/\/btc.foundryusapool.com:443","active":false,"user":"pcminervas.3.4x97","pass":false,"stats":[{"start_time":false,"accepted":2445,"rejected":43,"shares":30226,"stop_time":false,"stats_id":1}],"stats_id":1,"alive":1},{"priority":2,"url":"stratum+tcp:\/\/btc.foundryusapool.com:25","active":false,"user":"pcminervas.3.4x97","pass":false,"stats":[{"start_time":false,"accepted":547,"rejected":19,"shares":42224,"stop_time":false,"stats_id":1}],"stats_id":1,"alive":1}],"network_miners":[],"minera_id":"muzdbmowjinj","mac_addr":"00:24:9A:D5:CE:98","ifconfig":{"mac":"00:24:9A:D5:CE:98","mask":"255.255.254.0","ip":"10.24.4.97","gw":"10.24.5.254","dns":"208.67.220.220","dhcp":"static"},"miner":null,"algo":"SHA-256","sysload":[6.11,6.36,6.34],"cron":null,"sysuptime":"1051964","temp":91.17333333333333,"altcoins_rates":{"error":"true"},"avg":{"1min":[],"5min":[],"1hour":[],"1day":[]},"profits":null,"livestat":true,"timestamp":1678305549}"#;
        let stats: RunningStats = serde_json::from_str(json).unwrap();
    }
}
