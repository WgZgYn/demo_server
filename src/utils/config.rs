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
    pub topic_status: String,
    pub topic_status_qos: u8,
    pub topic_events: String,
    pub topic_events_qos: u8,
}

#[derive(Deserialize)]
pub struct ActixConfig {
    pub ip: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct GlobalConfig {
    pub database: DataBaseConfig,
    pub mqtt: MqttConfig,
    pub actix: ActixConfig,
}

pub fn read_config() -> Result<GlobalConfig, Box<dyn Error>> {
    let config = serde_yaml::from_str::<GlobalConfig>(include_str!("../../config.yaml"))?;
    Ok(config)
}
