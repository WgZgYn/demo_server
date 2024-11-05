use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
pub struct DataBaseConfig {
    pub host: String,
    pub dbname: String,
    pub user: String,
}

#[derive(Deserialize)]
pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub id: String,
    pub cap: usize,
    pub keep_alive: u64,
}

#[derive(Deserialize)]
pub struct GlobalConfig {
    pub database: DataBaseConfig,
    pub mqtt: MqttConfig,
}

pub fn read_config() -> Result<GlobalConfig, Box<dyn Error>> {
    let config = serde_yaml::from_str::<GlobalConfig>(include_str!("../../config.yaml"))?;
    Ok(config)
}