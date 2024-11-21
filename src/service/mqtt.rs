use crate::data::sse::SseHandler;
use crate::db::{CachedDataBase, Memory};
use crate::dto::mqtt::{DeviceMessage, HostMessage};
use crate::service::event::{Action, Trigger};
use crate::utils::config::MqttConfig;
use actix_web::web;
use log::{error, info};
use rumqttc::{AsyncClient, ClientError, Event, EventLoop, Incoming, MqttOptions, QoS};
use std::time::Duration;
use tokio::sync::RwLock;

pub async fn handle_mqtt_message(
    mut event_loop: EventLoop,
    sse_handler: web::Data<RwLock<SseHandler>>,
    memory: web::Data<Memory>,
    db: web::Data<CachedDataBase>,
    mqtt: web::Data<AsyncClient>,
) {
    loop {
        match event_loop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(publish))) => {
                println!("mqtt publish: {:?}", publish);
                let msg: DeviceMessage = match serde_json::from_slice(&publish.payload) {
                    Ok(msg) => msg,
                    Err(e) => {
                        error!("mqtt message parse error: {:?}", e);
                        continue;
                    }
                };

                let device_id: i32 = {
                    let session = match db.get_session().await {
                        Ok(session) => session,
                        Err(e) => {
                            error!("mqtt message db error: {:?}", e);
                            continue;
                        }
                    };

                    match session.get_device_id_by_mac(&msg.efuse_mac).await {
                        Ok(device_id) => device_id,
                        Err(e) => {
                            error!("mqtt message device_id error: {:?}", e);
                            continue;
                        }
                    }
                };

                match msg.type_.as_str() {
                    "status" => {
                        let mut guard = memory.device_state.write().await;
                        guard.on_device_status(device_id, msg.payload);

                        let guard = db.cache.device_id2account_ids.read().await;
                        if let Some(v) = guard.get(&device_id) {
                            for account_id in v {
                                let mut sse = sse_handler.write().await;
                                sse.send(*account_id, device_id.to_string().as_str()).await;
                            }
                        }
                    }
                    "event" => {
                        let mut guard = memory.device_state.write().await;
                        guard.on_device_event(device_id, msg.payload.clone());
                        let guard = memory.scenes.read().await;
                        let trigger = Trigger {
                            efuse_mac: msg.efuse_mac,
                            payload: msg.payload,
                        };

                        for scene in guard.iter() {
                            if let Some(actions) = scene.trigger(&trigger) {
                                for action in actions {
                                    if let Err(e) =
                                        execute_action(action.clone(), db.clone(), mqtt.clone())
                                            .await
                                    {
                                        error!("execute_action error: {:?}", e);
                                    }
                                }
                            }
                        }
                    }
                    t => {
                        info!("mqtt message type: {}", t);
                    }
                }
            }
            Err(e) => {
                error!("{}", e);
            }
            _ => continue,
        }
    }
}

/// action:
///     device_id: i32
///     service_name: String
///     body: String

pub async fn execute_action(
    action: Action,
    db: web::Data<CachedDataBase>,
    mqtt: web::Data<AsyncClient>,
) -> Result<(), Box<dyn std::error::Error>> {
    let session = db.get_session().await?;
    let mac = session.get_device_mac_by_id(action.device_id).await?;
    if action.body == "" {
        send_host_message(mqtt, &mac, HostMessage::none(action.service_name)).await?;
    } else {
        send_host_message(
            mqtt,
            &mac,
            HostMessage::text(action.service_name, action.body.into_bytes()),
        )
        .await?;
    }
    Ok(())
}

pub async fn send_host_message(
    mqtt: web::Data<AsyncClient>,
    efuse_mac: &str,
    message: HostMessage,
) -> Result<(), ClientError> {
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
