use serde::{Deserialize, Deserializer};

#[derive(PartialEq, Debug)]
pub enum StatusCode {
    WARN,
    INFO,
    SUCC,
    ERROR,
    FATAL,
}

impl<'de> Deserialize<'de> for StatusCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "W" => Ok(StatusCode::WARN),
            "I" => Ok(StatusCode::INFO),
            "S" => Ok(StatusCode::SUCC),
            "E" => Ok(StatusCode::ERROR),
            "F" => Ok(StatusCode::FATAL),
            _ => Err(serde::de::Error::custom(format!("Unknown status code: {}", s))),
        }
    }
}

pub enum RespCode {
}

#[derive(Deserialize, Debug)]
pub struct Status {
    #[serde(rename = "STATUS")]
    pub status: StatusCode,
    #[serde(rename = "When")]
    pub when: usize,
    #[serde(rename = "Code")]
    pub code: usize,
    #[serde(rename = "Msg")]
    pub msg: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct StatusResp {
    #[serde(rename = "STATUS")]
    pub status: [Status; 1],
}