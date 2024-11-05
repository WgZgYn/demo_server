use crate::utils::config::DataBaseConfig;
use deadpool_postgres::{Pool, Runtime};
use tokio_postgres::NoTls;

pub async fn create_connection_pool(config: &DataBaseConfig) -> Result<Pool, Box<dyn std::error::Error>> {
    let pass = std::env::var("DATABASE_PASSWORD").expect("need DATABASE_PASSWORD");
    let mut cfg = deadpool_postgres::Config::new();
    cfg.host = Some(config.host.clone());
    cfg.user = Some(config.user.clone());
    cfg.dbname = Some(config.dbname.clone());
    cfg.password = Some(pass);
    Ok(cfg.create_pool(Some(Runtime::Tokio1), NoTls)?)
}
