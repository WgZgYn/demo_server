use serde::Deserialize;

#[derive(Deserialize)]
pub struct AccountUpdate {}

#[derive(Deserialize)]
pub struct HouseUpdate {}

#[derive(Deserialize)]
pub struct AreaUpdate {}

#[derive(Deserialize)]
pub struct DeviceUpdate {}

#[derive(Deserialize)]
pub struct AccountLogin {}

#[derive(Deserialize)]
pub struct AccountSignup {}


#[derive(Deserialize)]
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
