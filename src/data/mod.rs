use actix_web::web::{Data, ServiceConfig};

pub mod sse;
pub mod sse_config;

pub fn config_appdata(cfg: &mut ServiceConfig) {
    let counter = Data::new(0);
    cfg.app_data(counter.clone());
}
