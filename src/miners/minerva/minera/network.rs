use serde::Deserialize;

#[derive(Deserialize)]
pub struct Ifconfig {
    pub mac: String,
    pub mask: String,
    pub ip: String,
    pub gw: String,
    pub dns: String,
    pub dhcp: String,
}