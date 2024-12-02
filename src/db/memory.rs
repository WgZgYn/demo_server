use crate::service::event::{Action, Scene, Trigger};
use log::error;
use rumqttc::{AsyncClient, ClientError, QoS};
use std::collections::{HashMap, VecDeque};
use tokio::sync::RwLock;
use crate::dto::mqtt::{DeviceMessage, HostToDeviceMessage};

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

#[derive(Default)]
pub struct Memory {
    pub device_state: DeviceState,
    pub scenes: SceneManager,
}

impl DeviceState {
    pub async fn status(&self, device_id: i32) -> Option<serde_json::Value> {
        let guard = self.status.read().await;
        guard.get(&device_id).cloned()
    }

    pub async fn on_device_message(
        &self,
        device_id: i32,
        message: DeviceMessage) -> Option<i32>
    {
        {
            let mut guard = self.online.write().await;
            guard.insert(device_id, message.efuse_mac.clone());
        };

        match message.type_.as_str() {
            "status" => {
                self.on_device_status(device_id, message.payload).await;
                Some(device_id)
            }
            "event" => {
                self.on_device_event(device_id, message.payload).await;
                None
            }
            s => {
                error!("not supported message type {s}");
                None
            }
        }
    }
    async fn on_device_status(&self, device_id: i32, status: serde_json::Value) {
        let mut guard = self.status.write().await;
        guard.insert(device_id, status);
    }
    async fn on_device_event(&self, device_id: i32, event: serde_json::Value) {
        let mut guard = self.events.write().await;
        guard.entry(device_id).or_insert(VecDeque::new()).push_back(event);
    }
    pub async fn update_all_devices_status(&mut self, client: &mut AsyncClient) {
        let message = HostToDeviceMessage::new("status".to_string(), None);
        if let Err(e) = client.publish("/device", QoS::AtLeastOnce, false, message).await {
            error!("error publishing device message {e}");
        }
    }

    pub async fn update_device_status(
        device_mac: &str,
        client: &mut AsyncClient,
    ) -> Result<(), ClientError> {
        let message = HostToDeviceMessage::new("status".to_string(), None);

        client
            .publish(
                format!("/device/{}/service", device_mac),
                QoS::AtLeastOnce,
                false,
                message,
            )
            .await
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
