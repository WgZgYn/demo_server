use std::time::Duration;
use actix_web::web;
use deadpool_postgres::Pool;
use log::error;
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};
use tokio::sync::RwLock;
use crate::data::device::{Cache, DeviceStatus};
use crate::data::sse::SseHandler;
use crate::utils::config::MqttConfig;

pub async fn handle_mqtt_message(
    mut event_loop: EventLoop,
    sse_handler: web::Data<RwLock<SseHandler>>,
    status: web::Data<RwLock<DeviceStatus>>,
    pool: web::Data<Pool>,
    cache: web::Data<RwLock<Cache>>
) {
    loop {
        match event_loop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(publish))) => {
                println!("mqtt publish: {:?}", publish);
                let msg: serde_json::Value = match serde_json::from_slice(&publish.payload) {
                    Ok(msg) => msg,
                    Err(e) => {
                        error!("mqtt message parse error: {:?}", e);
                        continue;
                    }
                };
            }
            Err(e) => {
                error!("{}", e);
            }
            _ => continue
        }
    }
}

pub async fn mqtt(cfg: &MqttConfig) -> (AsyncClient, EventLoop) {
    let mut options = MqttOptions::new(&cfg.id, &cfg.host, cfg.port);
    options.set_keep_alive(Duration::from_secs(cfg.keep_alive));
    let (client, event_loop) = AsyncClient::new(options, cfg.cap);
    client.subscribe(&cfg.topic_status, qos_from_u8(cfg.topic_status_qos)).await.unwrap();
    client.subscribe(&cfg.topic_events, qos_from_u8(cfg.topic_events_qos)).await.unwrap();
    (client, event_loop)
}

fn qos_from_u8(value: u8) -> QoS {
    match value {
        0 => QoS::AtMostOnce,
        1 => QoS::AtLeastOnce,
        2 => QoS::ExactlyOnce,
        _ => QoS::ExactlyOnce,
    }
}