use crate::service::event::Scene;
use log::error;
use rumqttc::{AsyncClient, ClientError, QoS};
use std::collections::{HashMap, VecDeque};
use tokio::sync::RwLock;

#[derive(Debug, Default)]
pub struct DeviceState {
    online: HashMap<i32, String>,
    status: HashMap<i32, serde_json::Value>,
    events: HashMap<i32, VecDeque<serde_json::Value>>,
}

#[derive(Default)]
pub struct Memory {
    pub device_state: RwLock<DeviceState>,
    pub scenes: RwLock<Vec<Scene>>,
}

impl DeviceState {
    pub fn status(&self, device_id: i32) -> Option<&serde_json::Value> {
        self.status.get(&device_id)
    }

    pub async fn on_device_message(
        &mut self,
        device_id: i32,
        msg: serde_json::Value,
    ) -> Option<i32> {
        match msg["type"].as_str() {
            Some("status") => {
                self.on_device_status(device_id, msg["payload"].clone());
                Some(device_id)
            }
            Some("event") => {
                self.on_device_event(device_id, msg["payload"].clone());
                None
            }
            _ => None,
        }
    }
    pub fn on_device_status(&mut self, device_id: i32, status: serde_json::Value) {
        if let Some(mac) = status["device_status"].as_str() {
            self.online.insert(device_id, mac.to_string());
            self.status.insert(device_id, status);
        } else {
            return;
        }
    }
    pub fn on_device_event(&mut self, device_id: i32, event: serde_json::Value) {
        self.events
            .entry(device_id)
            .or_insert(VecDeque::new())
            .push_back(event);
    }
    pub async fn update_all_devices_status(&mut self, client: &mut AsyncClient) {
        for (_, mac) in &self.online {
            if let Err(e) = Self::update_device_status(mac, client).await {
                error!("Error updating device status: {}", e);
            }
        }
    }
    async fn update_device_status(
        device_mac: &str,
        client: &mut AsyncClient,
    ) -> Result<(), ClientError> {
        client
            .publish(
                format!("/device/{}/service", device_mac),
                QoS::AtLeastOnce,
                false,
                "status",
            )
            .await
    }
}
