use deadpool_postgres::{Pool, Runtime};
use log::info;
use serde::Deserialize;
use tokio_postgres::NoTls;

pub async fn create_connection_pool() -> Result<Pool, Box<dyn std::error::Error>> {
    #[derive(Deserialize, Debug)]
    struct Config {
        database: DataBaseConfig,
    }
    #[derive(Deserialize, Debug)]
    struct DataBaseConfig {
        host: String,
        dbname: String,
        user: String,
    }
    let config: Config = serde_yaml::from_str::<Config>(include_str!("../../config.yaml"))?;
    info!("{:?}", &config);
    let pass = std::env::var("DATABASE_PASSWORD").expect("need DATABASE_PASSWORD");

    let mut cfg = deadpool_postgres::Config::new();

    cfg.host = Some(config.database.host);
    cfg.user = Some(config.database.user);
    cfg.dbname = Some(config.database.dbname);
    cfg.password = Some(pass);

    Ok(cfg.create_pool(Some(Runtime::Tokio1), NoTls)?)
}
