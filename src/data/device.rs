use std::collections::{HashMap, HashSet, VecDeque};
use actix_web::web;
use deadpool_postgres::Pool;
use log::error;
use rumqttc::{AsyncClient, ClientError, QoS};
use serde::Deserialize;
use crate::db::device::get_device_id_by_mac;

#[derive(Default)]
pub struct AccountDeviceStatus {
    account: HashMap<i32, DeviceStatus>,
    device_account_cache: HashMap<i32, HashSet<i32>>,
}

#[derive(Default)]
pub struct Cache {
    account_device: HashMap<i32, HashSet<i32>>,
    device_account: HashMap<i32, HashSet<i32>>,
    efuse_id: HashMap<String, i32>,
}

#[derive(Debug, Default)]
pub struct DeviceStatus {
    online: HashMap<i32, String>,
    status: HashMap<i32, serde_json::Value>,
    events: HashMap<i32, VecDeque<serde_json::Value>>,
}

impl DeviceStatus {
    pub fn online(&self, device_id: i32) -> bool {
        self.online.contains_key(&device_id)
    }
    pub fn status(&self, device_id: i32) -> Option<&serde_json::Value> {
        self.status.get(&device_id)
    }

    pub async fn on_device_message(&mut self, device_id: i32, msg: serde_json::Value) -> Option<i32> {
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
    fn on_device_status(&mut self, device_id: i32, status: serde_json::Value) {
        if let Some(mac) = status["device_status"].as_str() {
            self.online.insert(device_id, mac.to_string());
            self.status.insert(device_id, status);
        } else {
            return;
        }
    }
    fn on_device_event(&mut self, device_id: i32, event: serde_json::Value) {
        self.events.entry(device_id).or_insert(VecDeque::new()).push_back(event);
    }
    pub async fn update_all_devices_status(&mut self, client: &mut AsyncClient) {
        for (_, mac) in self.online.drain() {
            if let Err(e) = Self::update_device_status(mac, client).await {
                error!("Error updating device status: {}", e);
            }
        }
    }
    async fn update_device_status(device_mac: String, client: &mut AsyncClient) -> Result<(), ClientError> {
        client.publish(format!("/device/{}/service", device_mac), QoS::AtLeastOnce, false, "status").await
    }
}
