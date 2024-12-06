use demo_server::db::{create_connection_pool, CachedDataBase, DataBase, Memory};

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use demo_server::api::config_api;
use demo_server::data::sse::SseHandler;
use demo_server::security::{config_ssl, RecordIP};
use demo_server::service::middleware::Timer;
use demo_server::service::{handle_mqtt_message, mqtt};
use demo_server::utils::config::read_config;
use log::{debug, LevelFilter};
use tokio::sync::{Mutex, RwLock};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    debug!("starting server");

    let cfg = read_config()?;
    let ssl = config_ssl()?;

    let pool = create_connection_pool(&cfg.database).await?;
    let database = web::Data::new(DataBase::from(pool.clone()));
    let cached = web::Data::new(CachedDataBase::from(pool));
    let memory = web::Data::new(Memory::default());

    // 内存共享数据
    let counter = web::Data::new(Mutex::new(0));
    let sse_session = web::Data::new(RwLock::new(SseHandler::default()));
    let (client, event_loop) = mqtt(&cfg.mqtt).await;
    let client = web::Data::new(client);

    let sse1 = sse_session.clone();
    let memory1 = memory.clone();
    let database1 = cached;
    let client1 = client.clone();

    tokio::spawn(async move {
        handle_mqtt_message(event_loop, sse1, memory1, database1, client1).await;
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .wrap(Timer)
            .wrap(RecordIP::default())
            .app_data(counter.clone())
            .app_data(client.clone())
            .app_data(database.clone())
            .app_data(memory.clone())
            .app_data(sse_session.clone())
            .configure(config_api)
        // .configure(config_web)
        // .configure(config_redirects)
    })
        .bind(format!(
            "{ip}:{port}",
            ip = &cfg.actix.ip,
            port = &cfg.actix.port
        ))?
        .bind_openssl(format!("{}:443", &cfg.actix.ip), ssl)?
        .run()
        .await?;
    Ok(())
}
