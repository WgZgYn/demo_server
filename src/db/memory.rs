use crate::service::event::{Action, Scene, Trigger};
use log::{debug, error, info};
use rumqttc::{AsyncClient, ClientError, QoS};
use std::collections::{HashMap, VecDeque};
use actix_web::web;
use tokio::sync::RwLock;
use crate::data::sse::SseHandler;
use crate::db::{CachedDataBase, DataBase};
use crate::dto::mqtt::{DeviceMessage, HostToDeviceMessage};
use crate::service::execute_action;

#[derive(Debug, Default)]
pub struct DeviceState {
    online: RwLock<HashMap<i32, String>>,
    status: RwLock<HashMap<i32, serde_json::Value>>,
    events: RwLock<HashMap<i32, VecDeque<serde_json::Value>>>,
}


#[derive(Default)]
pub struct SceneManager {
    scenes: RwLock<Vec<Scene>>,
}

pub struct Memory {
    pub device_state: DeviceState,
    pub scenes: SceneManager,
    pub db: web::Data<CachedDataBase>,
}

impl Memory {
    pub fn new(db: web::Data<CachedDataBase>) -> Memory {
        Memory {
            device_state: DeviceState::default(),
            scenes: SceneManager::default(),
            db,
        }
    }

    pub async fn handle_device_message(
        &self,
        message: DeviceMessage,
        mqtt: web::Data<AsyncClient>,
        sse_handler: web::Data<RwLock<SseHandler>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let session = self.db.get_session().await?;
        let device_id: i32 = session.get_device_id_by_mac(&message.efuse_mac).await?;
        // record status in the memory
        self.device_state.on_device_message(device_id, message.clone()).await;
        match message.type_.as_str() {
            "status" => {
                // store status to database
                session.update_device_status(device_id, message.payload).await?;

                // handle sse session
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
                // store event to database
                session.record_device_event(device_id, message.payload.clone()).await?;


                // event trigger actions
                let trigger = Trigger {
                    efuse_mac: message.efuse_mac.clone(),
                    payload: message.payload.clone(),
                };

                let actions = self.scenes.try_trigger(trigger).await;
                for action in actions {
                    if let Err(e) = execute_action(action.clone(), self.db.clone(), mqtt.clone()).await {
                        error!("execute_action error: {:?}", e);
                    }
                }
            }

            t => {
                info!("mqtt message type: {} not support", t);
            }
        }
        Ok(())
    }
}

impl DeviceState {
    pub async fn status(&self, device_id: i32) -> Option<serde_json::Value> {
        let guard = self.status.read().await;
        guard.get(&device_id).cloned()
    }

    pub async fn on_device_message(
        &self,
        device_id: i32,
        message: DeviceMessage)
    {
        {
            let mut guard = self.online.write().await;
            guard.insert(device_id, message.efuse_mac.clone());
        };

        match message.type_.as_str() {
            "status" => {
                self.on_device_status(device_id, message.payload).await;
            }
            "event" => {
                self.on_device_event(device_id, message.payload).await;
            }
            s => {
                error!("not supported message type {s}");
            }
        }
    }
    async fn on_device_status(&self, device_id: i32, status: serde_json::Value) {
        info!("update device status {} {:?}", device_id, &status);
        let mut guard = self.status.write().await;
        guard.insert(device_id, status);
    }
    async fn on_device_event(&self, device_id: i32, event: serde_json::Value) {
        info!("get device event {} {:?}", device_id, &event);
        let mut guard = self.events.write().await;
        guard.entry(device_id).or_insert(VecDeque::new()).push_back(event);
    }
}


impl SceneManager {
    pub async fn try_trigger(&self, trigger: Trigger) -> Vec<Action> {
        let mut actions = Vec::new();
        let guard = self.scenes.read().await;
        for scene in guard.iter() {
            if let Some(v) = scene.trigger(&trigger) {
                actions.extend_from_slice(v);
            }
        }
        actions
    }
}
