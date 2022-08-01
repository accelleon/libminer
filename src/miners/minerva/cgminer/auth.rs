use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AuthData {
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthResp {
    pub code: usize,
    pub data: AuthData,
    pub message: String,
}