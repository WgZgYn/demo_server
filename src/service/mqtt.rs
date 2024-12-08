use crate::data::sse::SseHandler;
use crate::db::{CachedDataBase, Memory};
use crate::dto::mqtt::{DeviceMessage, HostToDeviceMessage};
use crate::service::event::{Action, Trigger};
use crate::utils::config::MqttConfig;
use actix_web::web;
use log::{debug, error, info};
use rumqttc::{AsyncClient, ClientError, Event, EventLoop, Incoming, MqttOptions, QoS};
use std::error::Error;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::Interval;

pub async fn handle_mqtt_message(
    mut event_loop: EventLoop,
    sse_handler: web::Data<RwLock<SseHandler>>,
    memory: web::Data<Memory>,
    mqtt: web::Data<AsyncClient>,
) {
    let interval = Duration::from_secs(15);
    let sender = mqtt.clone();
    tokio::spawn(async move {
        loop {
            let sender = sender.clone();
            update_all_devices_status(sender).await;
            tokio::time::sleep(interval).await;
        }
    });

    loop {
        match event_loop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(publish))) => {
                let message: DeviceMessage = match serde_json::from_slice(&publish.payload) {
                    Ok(message) => message,
                    Err(e) => {
                        error!("mqtt mqtt message parse error: {:?}", e);
                        continue;
                    }
                };
                {
                    let memory = memory.clone();
                    let sse_handler = sse_handler.clone();
                    let mqtt = mqtt.clone();
                    tokio::spawn(async move {
                        match handle_device_message(message, memory, sse_handler, mqtt).await {
                            Ok(_) => {
                                info!("mqtt message handled successfully");
                            }
                            Err(e) => {
                                error!("mqtt message handled error: {:?}", e);
                            }
                        };
                    });
                }
            }
            Err(e) => {
                error!("{}", e);
            }
            _ => continue,
        }
    }
}

pub async fn update_all_devices_status(client: web::Data<AsyncClient>) {
    let message = HostToDeviceMessage::status();
    if let Err(e) = client
        .publish("/device", QoS::AtLeastOnce, false, message)
        .await
    {
        error!("error publishing device message {e}");
    }
}

pub async fn update_device_status(
    device_mac: &str,
    client: web::Data<AsyncClient>,
) -> Result<(), ClientError> {
    let message = HostToDeviceMessage::status();
    client
        .publish(
            format!("/device/{}/service", device_mac),
            QoS::AtLeastOnce,
            false,
            message,
        )
        .await
}

async fn handle_device_message(
    msg: DeviceMessage,
    memory: web::Data<Memory>,
    sse_handler: web::Data<RwLock<SseHandler>>,
    mqtt: web::Data<AsyncClient>,
) -> Result<(), Box<dyn Error>> {
    info!("handle device message from device_mac: {}", &msg.efuse_mac);
    memory
        .handle_device_message(msg.clone(), mqtt, sse_handler)
        .await?;
    Ok(())
}

pub async fn execute_action(
    action: Action,
    db: web::Data<CachedDataBase>,
    mqtt: web::Data<AsyncClient>,
) -> Result<(), Box<dyn std::error::Error>> {
    let session = db.get_session().await?;
    let mac = session.get_device_mac_by_id(action.device_id).await?;

    let message = HostToDeviceMessage::new(action.service_name, action.body);
    send_host_message(mqtt, &mac, message).await?;
    Ok(())
}

pub async fn send_host_message(
    mqtt: web::Data<AsyncClient>,
    efuse_mac: &str,
    message: HostToDeviceMessage,
) -> Result<(), ClientError> {
    debug!("Sending host message: {:#?}", message);
    mqtt.publish(
        format!("/device/{}/service", efuse_mac),
        QoS::ExactlyOnce,
        false,
        message,
    )
    .await
}

pub async fn mqtt(cfg: &MqttConfig) -> (AsyncClient, EventLoop) {
    let mut options = MqttOptions::new(&cfg.id, &cfg.host, cfg.port);
    options.set_keep_alive(Duration::from_secs(cfg.keep_alive));
    let (client, event_loop) = AsyncClient::new(options, cfg.cap);
    client
        .subscribe(&cfg.topic_status, qos_from_u8(cfg.topic_status_qos))
        .await
        .unwrap();
    client
        .subscribe(&cfg.topic_events, qos_from_u8(cfg.topic_events_qos))
        .await
        .unwrap();
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
