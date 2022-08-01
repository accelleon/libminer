use serde::Deserialize;
pub use crate::miners::common::StatusCode;

mod stats;
pub use stats::*;
mod summary;
pub use summary::*;
mod sysinfo;
pub use sysinfo::*;
mod pools;
pub use pools::*;
mod conf;
pub use conf::*;

#[derive(Deserialize, Debug)]
pub struct Status {
    #[serde(rename = "STATUS")]
    pub status: StatusCode,
    pub when: usize,
    #[serde(rename = "Msg")]
    pub msg: String,
    pub api_version: String,
}

#[derive(Deserialize, Debug)]
pub struct CgiInfo {
    #[serde(rename = "CompileTime")]
    pub compile_time: String,
    pub miner_version: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Debug)]
pub struct CgiPostResp {
    pub code: String,
}