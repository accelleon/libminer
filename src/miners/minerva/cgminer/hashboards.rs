use serde::Deserialize;

fn deserialize_online<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "online" => Ok(true),
        "offline" => Ok(false),
        _ => Err(serde::de::Error::custom(format!("Unknown status code: {}", s))),
    }
}

#[derive(Deserialize, Debug)]
pub struct Hashboard {
    pub id: u32,
    #[serde(rename = "status", deserialize_with = "deserialize_online")]
    pub online: bool,
    pub temperature: f32,
}

#[derive(Deserialize, Debug)]
pub struct HashBoardsResp {
    pub code: usize,
    pub message: String,
    pub data: Option<Vec<Hashboard>>,
}
