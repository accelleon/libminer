use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LedStatus {
    pub status: String,
}

#[derive(Deserialize)]
pub struct LedResp {
    pub code: usize,
    pub data: LedStatus,
    pub message: String,
}
