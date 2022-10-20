use serde::Deserialize;

#[derive(Deserialize)]
pub struct LogResp {
    pub code: usize,
    pub data: Vec<String>,
    pub message: String,
}