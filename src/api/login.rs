use crate::api::auth::{create_token, Role};
use crate::utils;
use crate::utils::{hash, Response};
use actix_web::{web, HttpResponse};
use deadpool_postgres::{GenericClient, Pool};
use log::{error, info};
use serde::Deserialize;
use serde_json::json;

// 认证登录请求结构
#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

// 登录处理逻辑
pub async fn login(req: web::Json<LoginRequest>, db: web::Data<Pool>) -> HttpResponse {
    let username = &req.username;
    let password = &req.password;

    let client = match db.get().await {
        Ok(client) => client,
        Err(err) => {
            error!("{}", err);
            return HttpResponse::InternalServerError()
                .json(utils::Result::error("database error"));
        }
    };

    let row = match client
        .query_one(
            "SELECT password_hash, account_id FROM account WHERE username = $1",
            &[username],
        )
        .await
    {
        Ok(row) => row,
        Err(err) => {
            error!("{}", err);
            return HttpResponse::Unauthorized().json(utils::Result::error("no such user"));
        }
    };

    let password_hash = row.get(0);
    let account_id = row.get(1);

    let ok = hash::password_verify(password_hash, password.as_ref());
    if !ok {
        return HttpResponse::Unauthorized().json(utils::Result::error("password error"));
    }

    match client
        .execute(
            "UPDATE account SET last_login=CURRENT_TIMESTAMP WHERE username = $1",
            &[username],
        )
        .await
    {
        Ok(i) => {
            info!("{i} was updated");
            let token = create_token(username.clone(), Role::User, account_id);
            HttpResponse::Ok().json(Response::success(json!({"token": token,"role":"user"})))
        }
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError().json(utils::Result::error(
                "database error when update user's last_login",
            ))
        }
    }
}

pub async fn login_token() -> HttpResponse {
    HttpResponse::Ok().json(utils::Result::success())
}
