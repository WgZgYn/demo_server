use std::collections::{HashMap, VecDeque};
use log::error;
use rumqttc::{AsyncClient, ClientError, QoS};


#[derive(Debug, Default)]
pub struct DeviceStatus {
    online: HashMap<i32, String>,
    status: HashMap<i32, serde_json::Value>,
    events: HashMap<i32, VecDeque<String>>,
}

impl DeviceStatus {
    pub fn online(&self, device_id: i32) -> bool {
        self.online.contains_key(&device_id)
    }
    pub fn status(&self, device_id: i32) -> Option<&serde_json::Value> {
        self.status.get(&device_id)
    }
    pub fn on_device_status(&mut self, device_id: i32, status: serde_json::Value) {
        if let Some(mac) = status["device_status"].as_str() {
            self.online.insert(device_id, mac.to_string());
            self.status.insert(device_id, status);
        } else {
            return;
        }
    }
    pub fn on_device_event(&mut self, device_id: i32, event: String) {
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
