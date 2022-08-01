use serde::Deserialize;

use crate::miners::antminer::cgi::{Status, CgiInfo};

#[derive(Deserialize, Debug)]
pub struct StatusSummary {
    #[serde(rename = "type")]
    pub type_: String,
    pub status: String,
    pub code: i32,
    pub msg: String,
}

#[derive(Deserialize, Debug)]
pub struct Summary {
    pub bestshare: usize,
    pub elapsed: usize,
    pub hw_all: usize,
    pub rate_5s: f64,
    pub rate_30m: f64,
    pub rate_avg: f64,
    pub rate_ideal: f64,
    pub rate_unit: String,
    pub status: Vec<StatusSummary>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct SummaryResponse {
    pub info: CgiInfo,
    pub summary: Vec<Summary>,
    pub status: Status,
}