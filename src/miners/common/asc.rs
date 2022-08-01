use serde::Deserialize;

use crate::miners::common::{Status};

#[derive(Deserialize, Debug)]
pub struct AscIdentify {
    #[serde(rename = "Count")]
    pub count: usize,
}

#[derive(Deserialize, Debug)]
pub struct AscIdentifyResp {
    #[serde(rename = "STATUS")]
    pub status: [Status; 1],
    #[serde(rename = "ASCS")]
    pub ascs: [AscIdentify; 1],
}