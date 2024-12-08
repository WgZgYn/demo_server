use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeviceMessage {
    pub efuse_mac: String,
    pub model_id: i32,
    pub model_name: String,

    #[serde(rename = "type")]
    pub type_: String,
    pub payload: serde_json::Value,
}

#[derive(Serialize, Debug)]
pub struct HostToDeviceMessage {
    service_name: String,
    payload: Option<serde_json::Value>,
}

impl HostToDeviceMessage {
    pub fn new(service: String, body: Option<serde_json::Value>) -> HostToDeviceMessage {
        HostToDeviceMessage {
            service_name: service,
            payload: body,
        }
    }
    pub fn status() -> HostToDeviceMessage {
        HostToDeviceMessage::new("status".to_string(), None)
    }
}

impl Into<Vec<u8>> for HostToDeviceMessage {
    fn into(self) -> Vec<u8> {
        match serde_json::to_vec(&self) {
            Ok(bytes) => bytes,
            _ => Vec::new(),
        }
    }
}
