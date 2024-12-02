use std::error::Error;
use crate::data::sse::SseHandler;
use crate::db::{CachedDataBase, Memory};
use crate::dto::mqtt::{DeviceMessage, HostToDeviceMessage};
use crate::service::event::{Action, Trigger};
use crate::utils::config::MqttConfig;
use actix_web::web;
use log::{debug, error, info};
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
                info!("mqtt publish: {:?}", publish);
                debug!("message: {:#?}", publish.payload);
                let message: DeviceMessage = match serde_json::from_slice(&publish.payload) {
                    Ok(message) => message,
                    Err(e) => {
                        error!("mqtt mqtt message parse error: {:?}", e);
                        continue;
                    }
                };
                {
                    let db = db.clone();
                    let memory = memory.clone();
                    let sse_handler = sse_handler.clone();
                    let mqtt = mqtt.clone();
                    tokio::spawn(async move {
                        match handle_device_message(message, db, memory, sse_handler, mqtt).await {
                            Ok(_) => { info!("mqtt message handled successfully"); },
                            Err(e) => { error!("mqtt message handled error: {:?}", e); },
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


async fn handle_device_message(
    msg: DeviceMessage,
    db: web::Data<CachedDataBase>,
    memory: web::Data<Memory>,
    sse_handler: web::Data<RwLock<SseHandler>>,
    mqtt: web::Data<AsyncClient>,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("handle message: {:#?}", &msg);


    let session = db.get_session().await?;
    debug!("session ok");
    let device_id: i32 = session.get_device_id_by_mac(&msg.efuse_mac).await?;
    debug!("query ok");
    info!("device_id: {}", device_id);


    match msg.type_.as_str() {
        "status" => {
            let account_ids = session.get_account_ids_by_device_id(device_id).await?;
            debug!("account_ids: {:?}", account_ids);
            for account_id in account_ids {
                debug!("account_id: {}", account_id);
                let mut sse = sse_handler.write().await;
                info!("send sse message");
                sse.send(account_id, device_id.to_string().as_str()).await;
            }
        }

        "event" => {
            let trigger = Trigger {
                efuse_mac: msg.efuse_mac.clone(),
                payload: msg.payload.clone(),
            };

            let actions = memory.scenes.try_trigger(trigger).await;
            for action in actions {
                if let Err(e) = execute_action(action.clone(), db.clone(), mqtt.clone()).await {
                    error!("execute_action error: {:?}", e);
                }
            }
        }

        t => {
            info!("mqtt message type: {}", t);
        }
    }

    memory.device_state.on_device_message(device_id, msg).await;

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
