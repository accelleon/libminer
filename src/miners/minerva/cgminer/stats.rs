use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TempAndSpeed {
    #[serde(rename = "fan1Speed")]
    pub fan_speed1: u32,
    #[serde(rename = "fan2Speed")]
    pub fan_speed2: u32,
    pub temperature: f64,
}

#[derive(Deserialize, Debug)]
pub struct TempAndSpeedResp {
    pub code: usize,
    pub data: TempAndSpeed,
    pub message: String,
}