use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Network {
    pub dhcp4: bool,
    pub dns: String,
    pub dnsBak: String,
    pub gateway: String,
    pub hardwareAddress: String,
    pub interfaceName: String,
    pub ip: String,
    pub netmask: String,
}

#[derive(Deserialize, Debug)]
pub struct NetworkResponse {
    pub code: usize,
    pub data: Network,
    pub message: String,
}