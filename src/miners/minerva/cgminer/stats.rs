use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TempAndSpeedResp {
    #[serde(rename = "fan1Speed")]
    pub fan_speed1: u32,
    #[serde(rename = "fan2Speed")]
    pub fan_speed2: u32,
    pub temperature: f64,
}