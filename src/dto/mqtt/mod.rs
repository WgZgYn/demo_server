use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DeviceMessage {
    pub efuse_mac: String,
    pub model_id: i32,
    pub model_name: String,
    pub type_id: i32,

    #[serde(rename = "type")]
    pub type_: String,
    pub payload: serde_json::Value,
}

pub enum ContentType {
    None,
    Text,
    Json,
    File,
}

impl ContentType {
    fn index(&self) -> u8 {
        match self {
            ContentType::None => 0,
            ContentType::Text => 1,
            ContentType::Json => 2,
            ContentType::File => 3,
        }
    }
}

pub struct HostMessage {
    content_type: ContentType,
    content_length: i32,
    service_length: u8,
    service_name: Vec<u8>,
    body: Vec<u8>,
}

impl HostMessage {
    pub fn json(service_name: String, body: Vec<u8>) -> Self {
        Self {
            content_type: ContentType::Json,
            content_length: body.len() as i32,
            service_length: service_name.len() as u8,
            service_name: service_name.into_bytes(),
            body,
        }
    }

    pub fn text(service_name: String, body: Vec<u8>) -> Self {
        Self {
            content_type: ContentType::Text,
            content_length: body.len() as i32,
            service_length: service_name.len() as u8,
            service_name: service_name.into_bytes(),
            body,
        }
    }

    pub fn none(service_name: String) -> Self {
        Self {
            content_type: ContentType::None,
            content_length: 0,
            service_length: service_name.len() as u8,
            service_name: service_name.into_bytes(),
            body: vec![],
        }
    }

    pub fn file() {}

    pub fn bytes(self) -> Vec<u8> {
        let mut buffer =
            Vec::with_capacity((self.content_length + self.content_length + 7) as usize);
        buffer.push(self.content_type.index());
        buffer.extend_from_slice(&(self.content_length + 1).to_be_bytes());
        buffer.push(self.service_length + 1);
        let (service_name, body) = (&self.service_name, &self.body);
        buffer.extend_from_slice(&service_name);
        buffer.push(0);
        buffer.extend_from_slice(&body);
        buffer.push(0);

        for c in &buffer {
            println!("{} ", c);
        }

        buffer
    }
}

impl Into<Vec<u8>> for HostMessage {
    fn into(self) -> Vec<u8> {
        self.bytes()
    }
}
