use crate::dto::account::SignupAccount;
use crate::utils::hash::{gen_salt, password_hash};
use crate::utils::Response;
use actix_web::{web, HttpResponse};
use deadpool_postgres::Pool;
use log::error;
use crate::utils;

pub async fn signup(account: web::Json<SignupAccount>, pool: web::Data<Pool>) -> HttpResponse {
    let account = account.into_inner();
    let salt = gen_salt();
    let hash = password_hash(account.password(), &salt);
    match pool.get().await {
        Ok(conn) => {
            let ok = conn.execute(
                "INSERT INTO account \
                (username, password_hash, salt) \
                VALUES ($1, $2, $3)", &[&account.username(), &hash, &hex::encode(salt)]).await;

            match ok {
                Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
                Err(e) => {
                    error!("sql execute error {}", e);
                    HttpResponse::InternalServerError().json(utils::Result::error("sql execute error"))
                }
            }
        }
        Err(err) => {
            error!("client get error: {}", err);
            HttpResponse::InternalServerError().json(utils::Result::error("database error"))
        }
    }
}