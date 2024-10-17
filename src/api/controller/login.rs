use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{GenericClient, Pool};
use log::{error, info};
use serde::Deserialize;
use serde_json::json;
use crate::security::{Role, create_token};
use crate::utils;
use crate::utils::{hash, Response};

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
            return HttpResponse::InternalServerError().json(utils::Result::error("database error"));
        }
    };

    match client.query_one("SELECT password_hash FROM account WHERE username = $1", &[username]).await {
        Ok(row) => {
            let password_hash = row.get(0);
            let ok = hash::password_verify(password_hash, password.as_ref());
            if !ok {
                HttpResponse::Unauthorized()
                    .json(utils::Result::error("password error"))
            } else {
                match client.execute("UPDATE account SET last_login=CURRENT_TIMESTAMP WHERE username = $1", &[username]).await {
                    Ok(i) => {
                        info!("{i} was updated");
                        let token = create_token(username.clone(), Role::User);
                        HttpResponse::Ok().json(
                            Response::success(json!({"token": token,"role":"user"}))
                        )
                    }
                    Err(err) => {
                        error!("{}", err);
                        HttpResponse::InternalServerError().json(utils::Result::error("database error when update user's last_login"))
                    }
                }
            }
        }

        Err(err) => {
            error!("{}", err);
            HttpResponse::Unauthorized()
                .json(utils::Result::error("no such user"))
        }
    }
}