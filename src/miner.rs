use async_trait::async_trait;
use serde::Serialize;

use crate::error::Error;
use crate::Client;

// We can implement serialize directly on here for antminer
#[derive(Debug, Serialize)]
pub struct Pool {
    pub url: String,
    #[serde(rename = "user")]
    pub username: String,
    #[serde(rename = "pass")]
    pub password: Option<String>,
}

#[async_trait]
pub trait Miner {
    fn new(client: Client, ip: String, port: u16) -> Self
        where Self: Sized;

    fn get_type(&self) -> &'static str;

    async fn get_model(&self) -> Result<String, Error>;

    async fn auth(&mut self, username: &str, password: &str) -> Result<(), Error>;

    async fn reboot(&mut self) -> Result<(), Error>;

    async fn get_hashrate(&self) -> Result<f64, Error>;

    async fn get_temperature(&self) -> Result<f64, Error>;

    async fn get_fan_speed(&self) -> Result<Vec<u32>, Error>;

    async fn get_pools(&self) -> Result<Vec<Pool>, Error>;

    async fn set_pools(&mut self, pools: Vec<Pool>) -> Result<(), Error>;

    async fn set_sleep(&mut self, sleep: bool) -> Result<(), Error>;

    async fn set_blink(&mut self, blink: bool) -> Result<(), Error>;

    async fn get_logs(&mut self) -> Result<Vec<String>, Error>;
}