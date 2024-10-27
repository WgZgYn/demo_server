use crate::db::DB;
use actix_web::web;
use actix_web::web::{Data, ServiceConfig};
use deadpool_postgres::Pool;

pub mod sse_config;

pub fn config_appdata(cfg: &mut ServiceConfig) {
    let counter = web::Data::new(0);
    let db = web::Data::new(DB::default());
    
    cfg
        .app_data(counter.clone())
        .app_data(db.clone());
}

pub fn config_pool(pool: Data<Pool>) -> impl FnOnce(&mut ServiceConfig) {
    |cfg| {
        cfg.app_data(pool);
    }
}