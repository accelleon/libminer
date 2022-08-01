use serde::Deserialize;

use crate::miners::antminer::cgi::{Status, CgiInfo};

#[derive(Deserialize, Debug)]
pub struct PoolStat {
    pub accepted: usize,
    pub diff: String,
    pub diff1: usize,
    pub diffa: usize,
    pub diffr: usize,
    pub diffs: usize,
    pub discarded: usize,
    pub getworks: usize,
    pub index: usize,
    pub lsdiff: usize,
    pub lstime: String,
    pub priority: usize,
    pub rejected: usize,
    pub stale: usize,
    pub status: String,
    pub url: String,
    pub user: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct PoolsResponse {
    pub info: CgiInfo,
    pub pools: Vec<PoolStat>,
    pub status: Status,
}