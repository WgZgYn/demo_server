use demo_server::db::{create_connection_pool};
use demo_server::web::{config_redirects, config_web};

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use demo_server::api::config_api;
use demo_server::service::middleware::Timer;
use demo_server::security::{config_ssl, RecordIP};
use log::debug;
use tokio::sync::Mutex;
use demo_server::data::device::DeviceStatus;
use demo_server::service::{handle_mqtt_message, mqtt};
use demo_server::utils::config::read_config;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    debug!("starting server");

    let cfg = read_config()?;
    let ssl = config_ssl()?;
    let pool = web::Data::new(create_connection_pool(&cfg.database).await?);
    // 内存共享数据
    let counter = web::Data::new(Mutex::new(0));
    let memory = web::Data::new(DeviceStatus::default());
    let (client, event_loop) = mqtt(&cfg.mqtt);
    let client = web::Data::new(client);

    tokio::spawn(async move {
        handle_mqtt_message(event_loop).await;
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
