use demo_server::db::{create_connection_pool, DataBase};
use demo_server::web::{config_redirects, config_web};

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use demo_server::api::{config_api, config_api_v2};
use demo_server::data::device::{Cache, DeviceStatus};
use demo_server::data::sse::SseHandler;
use demo_server::security::{config_ssl, RecordIP};
use demo_server::service::middleware::Timer;
use demo_server::service::{handle_mqtt_message, mqtt};
use demo_server::utils::config::read_config;
use log::debug;
use tokio::sync::{Mutex, RwLock};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    debug!("starting server");

    let cfg = read_config()?;
    let ssl = config_ssl()?;

    let pool = create_connection_pool(&cfg.database).await?;

    let database = Data::new(DataBase::from(pool.clone()));
    let pool = web::Data::new(pool);

    // 内存共享数据
    let counter = web::Data::new(Mutex::new(0));
    let cache = web::Data::new(RwLock::new(Cache::default()));
    let memory = web::Data::new(RwLock::new(DeviceStatus::default()));
    let sse_session = web::Data::new(RwLock::new(SseHandler::default()));
    let (client, event_loop) = mqtt(&cfg.mqtt).await;
    let client = web::Data::new(client);

    let memory1 = memory.clone();
    let pool1 = pool.clone();
    let sse1 = sse_session.clone();

    tokio::spawn(async move {
        handle_mqtt_message(event_loop, sse1, memory1, pool1, cache).await;
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(Timer)
            .wrap(RecordIP::default())
            .app_data(counter.clone())
            .app_data(pool.clone())
            .app_data(client.clone())
            .app_data(memory.clone())
            .app_data(database.clone())
            // .configure(config_api)
            .configure(config_api_v2)
            // .configure(config_web) // vue static dist
            // .configure(config_redirects)
    })
    .bind("0.0.0.0:8123")?
    .bind_openssl("0.0.0.0:443", ssl)?
    .run()
    .await?;
    Ok(())
}
