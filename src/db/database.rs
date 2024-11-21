use crate::db::create_connection_pool;
use crate::utils::config::DataBaseConfig;
use deadpool_postgres::{Pool, PoolError};

pub struct Session(pub(crate) deadpool_postgres::Client);

pub struct DataBase {
    pool: Pool,
}

impl DataBase {
    pub async fn new(config: &DataBaseConfig) -> Self {
        Self {
            pool: create_connection_pool(config).await.unwrap(),
        }
    }
    pub fn from(pool: Pool) -> Self {
        Self { pool }
    }
    pub async fn get_session(&self) -> Result<Session, PoolError> {
        let client = self.pool.get().await?;
        Ok(Session(client))
    }
}

impl Session {}
