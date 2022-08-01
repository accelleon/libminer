use serde::Deserialize;

#[derive(Deserialize)]
pub struct ActivePool {
    pub hashrate: usize,
    pub url: String,
    pub user: String,
    pub alive: u8,
}

#[derive(Deserialize)]
pub struct PoolStats {
    pub start_time: bool,
    pub accepted: usize,
    pub rejected: usize,
    pub shares: usize,
    pub stop_time: bool,
    pub stats_id: usize,
}

#[derive(Deserialize)]
pub struct Pool {
    pub priority: usize,
    pub url: String,
    pub active: bool,
    pub user: String,
    pub pass: bool,
    pub stats: Vec<PoolStats>,
    pub stats_id: usize,
    pub alive: u8,
}