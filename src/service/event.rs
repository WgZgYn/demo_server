use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// When an Event was in the set of trigger, it will cause all the actions
#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub scene_id: i32,
    pub scene_name: String,
    pub house_id: String,
    pub triggers: HashSet<Trigger>,
    pub actions: Vec<Action>,
}
impl Scene {
    pub fn trigger(&self, trigger: &Trigger) -> Option<&Vec<Action>> {
        if self.triggers.contains(trigger) {
            Some(&self.actions)
        } else {
            None
        }
    }
}

// TODO:
#[derive(Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct Trigger {
    pub efuse_mac: String,
    pub payload: serde_json::Value,
}

pub struct TimeTrigger {}

#[derive(Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct TriggerEntity {
    pub trigger_type: String,
    pub data: serde_json::Value,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Action {
    pub device_id: i32,
    pub service_name: String,
    pub body: Option<serde_json::Value>,
}
