use demo_server::db::{create_connection_pool, DB};
use demo_server::web::{config_redirects, config_web};

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use demo_server::api::config_api;
use demo_server::middleware::Timer;
use demo_server::security::{config_ssl, RecordIP};
use log::debug;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    debug!("starting server");

    let ssl = config_ssl()?;
    let pool = web::Data::new(create_connection_pool().await?);

    // 内存共享数据
    let counter = web::Data::new(Mutex::new(0));
    let db = web::Data::new(DB::default());

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(Timer)
            .wrap(RecordIP::default())
            
            .app_data(counter.clone())
            .app_data(db.clone())
            .app_data(pool.clone())
            
            .configure(config_api)
            .configure(config_web) // vue static dist
            .configure(config_redirects)
    })
    .bind("0.0.0.0:80")?
    .bind_openssl("0.0.0.0:443", ssl)?
    .run()
    .await?;

    Ok(())
}
