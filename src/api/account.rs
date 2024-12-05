use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use log::{error, info};
use crate::db::DataBase;
use crate::dto::http::request::{AccountUpdate, Login, Signup, UserInfoUpdate};
use crate::dto::http::response::LoginSuccess;
use crate::security::auth::Claims;
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


pub async fn delete_account(req: HttpRequest, db: web::Data<DataBase>) -> HttpResponse {
    let e = req.extensions();
    let claims = match e.get::<Claims>() {
        Some(claims) => claims,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let mut session = match db.get_session().await {
        Ok(session) => session,
        Err(e) => { error!("{}", e); return HttpResponse::InternalServerError().finish(); }
    };

    match session.delete_account(claims.id()).await {
        Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
        Err(e) => { error!("{}", e); HttpResponse::InternalServerError().finish() }
    }
}

pub async fn update_account(req: HttpRequest, account: web::Json<AccountUpdate>, db: web::Data<DataBase>) -> HttpResponse {
    let e = req.extensions();
    let claims = match e.get::<Claims>() {
        Some(claims) => claims,
        None => return HttpResponse::Unauthorized().finish(),
    };

    let session = match db.get_session().await {
        Ok(session) => session,
        Err(e) => { error!("{}", e); return HttpResponse::InternalServerError().finish(); }
    };

    match session.update_account(account.into_inner(), claims.id()).await {
        Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
        Err(e) => { error!("{}", e); HttpResponse::InternalServerError().finish() }
    }
}


pub async fn get_user_info(req: HttpRequest, db: web::Data<DataBase>) -> HttpResponse {
    let e = req.extensions();
    let claims = match e.get::<Claims>() {
        Some(claims) => claims,
        None => return HttpResponse::Unauthorized().finish(),
    };
    match db.get_session().await {
        Ok(session) => match session.get_user_info(claims.id()).await {
            Ok(v) => HttpResponse::Ok().json(Response::success(v)),
            Err(e) => {
                error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_user_info(
    req: HttpRequest,
    data: web::Json<UserInfoUpdate>,
    db: web::Data<DataBase>,
) -> HttpResponse {
    let e = req.extensions();
    let claims = match e.get::<Claims>() {
        Some(claims) => claims,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match db.get_session().await {
        Ok(session) => {
            match session
                .update_user_info(data.into_inner(), claims.id())
                .await
            {
                Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
                Err(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}