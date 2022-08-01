mod auth;
pub use auth::*;
mod pool;
pub use pool::*;
mod summary;
pub use summary::*;
mod stats;
pub use stats::*;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum DataType {
}

#[derive(Deserialize, Debug)]
pub struct ApiResp {
    pub code: usize,
    pub message: String,
    pub data: Option<String>,
}