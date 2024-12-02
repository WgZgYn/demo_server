use actix_web::{web, HttpResponse};
use log::{error, info};
use crate::db::DataBase;
use crate::dto::http::request::{Login, Signup};
use crate::dto::http::response::LoginSuccess;
use crate::security::hash;
use crate::security::hash::{gen_salt, password_hash};
use crate::utils;
use crate::utils::Response;

pub async fn login(data: web::Json<Login>, db: web::Data<DataBase>) -> HttpResponse {
    let Login { username, password } = data.into_inner();
    let session = match db.get_session().await {
        Ok(session) => session,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let (account_id, password_hash) = match session.get_account_id_password_hash(&username).await {
        Ok(v) => v,
        Err(e) => {
            info!("{}", e);
            return HttpResponse::Unauthorized().json(utils::Result::error("No such Account"));
        }
    };

    if hash::password_verify(&password_hash, password.as_ref()) {
        let token = match session
            .update_account_last_login(account_id, username)
            .await
        {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };
        HttpResponse::Ok().json(Response::success(LoginSuccess { token }))
    } else {
        HttpResponse::Unauthorized().json(utils::Result::error("Wrong password"))
    }
}

pub async fn signup(data: web::Json<Signup>, db: web::Data<DataBase>) -> HttpResponse {
    let Signup { username, password } = data.into_inner();
    let session = match db.get_session().await {
        Ok(session) => session,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let salt = gen_salt();
    let hash = password_hash(&password, &salt);

    if let Err(e) = session.add_account(&username, &hash, &salt).await {
        error!("{}", e);
        return HttpResponse::InternalServerError().json(utils::Result::error("No such Account"));
    }

    HttpResponse::Ok().finish()
}