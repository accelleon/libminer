use serde::Deserialize;

#[derive(Deserialize)]
pub struct Device {
    pub temperature: f64,
    pub frequency: usize,
    pub accepted: usize,
    pub rejected: usize,
    pub hw_errors: usize,
    pub shares: usize,
    pub hashrate: usize,
    pub last_share: usize,
    pub serial: bool,
}

#[derive(Deserialize)]
pub struct Devices {
    #[serde(rename = "Board-1")]
    pub board_1: Option<Device>,
    #[serde(rename = "Board-2")]
    pub board_2: Option<Device>,
    #[serde(rename = "Board-3")]
    pub board_3: Option<Device>,
    #[serde(rename = "Board-4")]
    pub board_4: Option<Device>,
}

#[derive(Deserialize)]
pub struct DeviceTotal {
    pub temperature: f64,
    pub frequency: usize,
    pub accepted: usize,
    pub rejected: usize,
    pub hw_errors: usize,
    pub shares: usize,
    pub hashrate: usize,
    pub last_share: usize,
}