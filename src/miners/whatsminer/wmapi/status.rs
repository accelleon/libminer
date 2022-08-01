use serde::Deserialize;

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