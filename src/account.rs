use crate::db::DB;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq)]
pub struct Username(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub username: Username,
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
    let id = msg.account.username.clone();
    match msg.action {
        AccountAction::Create => match data.users.write() {
            Ok(mut users) => {
                users.insert(id, msg.account.clone());
            }
            Err(err) => {
                println!("{:#?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        },
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
    if let Ok(s) = data.users.read() {
        HttpResponse::Ok().json(&*s)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}
