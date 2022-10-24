use serde::Deserialize;
use std::collections::HashMap;

use crate::miners::common;

#[derive(Debug, Deserialize)]
pub struct ErrorData {
    pub error_code: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResp {
    #[serde(rename = "STATUS")]
    pub status: common::StatusCode,
    #[serde(rename = "When")]
    pub when: usize,
    #[serde(rename = "Code")]
    pub code: usize,
    #[serde(rename = "Msg")]
    pub msg: ErrorData,
    #[serde(rename = "Description")]
    pub description: String,
}
