use actix_web::web::{Data, ServiceConfig};
use deadpool_postgres::Pool;

pub mod device;
pub mod sse;
pub mod sse_config;

pub fn config_appdata(cfg: &mut ServiceConfig) {
    let counter = Data::new(0);
    cfg.app_data(counter.clone());
}

pub fn config_pool(pool: Data<Pool>) -> impl FnOnce(&mut ServiceConfig) {
    |cfg| {
        cfg.app_data(pool);
    }
}
