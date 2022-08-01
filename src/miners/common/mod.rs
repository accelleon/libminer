// Re-export our response types
mod status;
pub use status::*;
mod device;
pub use device::*;
mod pool;
pub use pool::*;
mod summary;
pub use summary::*;
mod stats;
pub use stats::*;
mod asc;
pub use asc::*;

use serde::Deserialize;

// We ship a bulk command for as much info as possible
#[derive(Deserialize, Debug)]
pub struct BulkResponse {
    pub summary: [SummaryResp; 1],
    pub pools: [PoolsResp; 1],
    pub devs: [DevsResp; 1],
    pub stats: [StatsResp; 1],
}