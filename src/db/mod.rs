mod cache_database;
mod data;
mod database;
mod memory;
pub mod pool;

use tokio_postgres::Row;

pub use pool::create_connection_pool;

pub type QueryType = Result<Vec<Row>, tokio_postgres::Error>;
pub type QueryOneType = Result<Row, tokio_postgres::Error>;
pub type ExecuteType = Result<u64, tokio_postgres::Error>;

pub use cache_database::CachedDataBase;
pub use database::DataBase;
pub use memory::Memory;
