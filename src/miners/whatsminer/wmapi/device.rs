use serde::Deserialize;
use crate::miners::common;
use crate::miners::whatsminer::wmapi;

#[derive(Deserialize, Debug)]
pub struct DevDetailsResp {
    #[serde(rename = "STATUS")]
    pub status: Vec<wmapi::Status>,
    #[serde(rename = "DEVDETAILS")]
    pub devdetails: Vec<common::DevDetails>,
}