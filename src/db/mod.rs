pub mod area;
pub mod device;
pub mod house;
pub mod pool;
pub mod user_info;

use tokio_postgres::Row;

pub use pool::create_connection_pool;

pub type QueryType = Result<Vec<Row>, tokio_postgres::Error>;
pub type QueryOneType = Result<Row, tokio_postgres::Error>;

pub type ExecuteType = Result<u64, tokio_postgres::Error>;
