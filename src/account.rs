use crate::db::DB;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq)]
pub struct UserId(String); // This is the device's MAC address

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub user_id: UserId,
    password_hash: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum AccountAction {
    Create,
    Update,
    Delete,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccountQuery {
    account: Account,
    action: AccountAction,
}

pub async fn post_account(data: web::Data<DB>, msg: web::Json<AccountQuery>) -> HttpResponse {
    println!("{:?}", msg.0);
    let id = msg.account.user_id.clone();
    match msg.action {
        AccountAction::Create => {
            let mut guard = data.users.write().unwrap();
            guard.insert(id, msg.account.clone());
        }
        AccountAction::Update => {}
        AccountAction::Delete => {}
    }
    HttpResponse::Ok().json(json!(
    {
        "status": "ok",
        "action": msg.action,
    }))
}

pub async fn get_account(data: web::Data<DB>) -> HttpResponse {
    HttpResponse::Ok().json(&*data.users.read().expect("read error"))
}
