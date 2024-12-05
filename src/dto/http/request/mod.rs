use crate::service::event::{Action, Trigger};
use serde::Deserialize;
use std::collections::HashSet;
#[derive(Deserialize)]
pub struct HouseUpdate {
    pub house_name: String
}

#[derive(Deserialize)]
pub struct AreaUpdate {
    pub area_name: String,
}

#[derive(Deserialize)]
pub struct DeviceUpdate {
    pub device_name: Option<String>,
    pub area_id: Option<i32>
}

#[derive(Deserialize)]
pub struct AccountUpdate {
    pub account_name: Option<String>,
    pub old_password: String,
    pub new_password: Option<String>,
}

#[derive(Deserialize)]
pub struct SceneAdd {
    pub scene_name: String,
    pub house_id: i32,
    pub triggers: HashSet<Trigger>,
    pub actions: Vec<Action>,
}

#[derive(Deserialize)]
pub struct AreaAdd {
    pub area_name: String,
    pub house_id: i32,
}

#[derive(Deserialize)]
pub struct HouseAdd {
    pub house_name: String,
}

#[derive(Deserialize, Debug)]
pub struct UserInfoUpdate {
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub city: Option<String>,
}

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Signup {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct MemberDelete {
    pub account_id: i32,
    pub house_id: i32,
}

#[derive(Deserialize)]
pub struct MemberAdd {
    pub account_id: i32,
    pub house_id: i32,
}

