use serde::{Deserialize, de};

pub use crate::miners::common::StatusCode;

#[derive(Deserialize, Debug)]
pub struct Status {
    #[serde(rename = "STATUS")]
    pub status: StatusCode,
    #[serde(rename = "When")]
    pub when: Option<usize>,
    #[serde(rename = "Code")]
    pub code: Option<usize>,
    #[serde(rename = "Msg")]
    pub msg: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(de::Error::custom(format!("invalid bool string: {}", s))),
    }
}

#[derive(Deserialize, Debug)]
pub struct BtStatus {
    #[serde(deserialize_with = "deserialize_bool")]
    pub btmineroff: bool,
    #[serde(rename = "Firmware Version", alias = "FirmwareVersion")]
    pub firmware_version: String,
}

#[derive(Deserialize, Debug)]
pub struct BtStatusResp {
    #[serde(rename = "STATUS")]
    pub status: StatusCode,
    #[serde(rename = "When")]
    pub when: Option<usize>,
    #[serde(rename = "Code")]
    pub code: Option<usize>,
    #[serde(rename = "Msg")]
    pub msg: BtStatus,
    #[serde(rename = "Description")]
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bt_status() {
        let json = r#"{"btmineroff":"true","Firmware Version":"1.0.0"}"#;
        let status: BtStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.btmineroff, true);
        assert_eq!(status.firmware_version, "1.0.0");
    }
}
