use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SystemInfoResponse {
    pub minertype: String,
    pub nettype: String,
    pub netdevice: String,
    pub macaddr: String,
    pub hostname: String,
    pub ipaddress: String,
    pub netmask: String,
    pub gateway: String,
    pub dnsservers: String,
    pub system_mode: String,
    pub system_kernel_version: String,
    pub system_filesystem_version: String,
    pub firmware_type: String,
}