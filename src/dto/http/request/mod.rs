use serde::{Deserialize, Serialize};

pub struct AccountUpdate {

}

pub struct HouseUpdate {

}

pub struct AreaUpdate {

}

pub struct DeviceUpdate {

}

pub struct AccountLogin {

}

pub struct AccountSignup {

}

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Signup {
    pub username: String,
    pub password: String
}