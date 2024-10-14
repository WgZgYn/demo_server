use actix_web::{web, Error, HttpResponse};
use log::info;
use serde::Deserialize;
use serde_json::json;
use crate::security::{Role, create_token};
use crate::utils::Response;

// 认证登录请求结构
#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

// 登录处理逻辑
pub async fn login(req: web::Json<LoginRequest>) -> Result<HttpResponse, Error> {
    let username = &req.username;
    let password = &req.password;

    info!("username: {}", &username);
    info!("password: {}", &password);

    // 在这里处理用户认证逻辑，例如验证用户名和密码是否正确
    // 如果用户名和密码正确，则生成 JWT 令牌
    if username == "admin" && password == "123456" {
        let token = create_token(username.clone(), Role::Admin);
        return Ok(HttpResponse::Ok().json(
            Response::success(json!({"token": token,"role":"admin"}))
        ));  // 返回 JWT
    }

    if username == "wzy" && password == "123456" {
        let token = create_token(username.clone(), Role::User);
        return Ok(HttpResponse::Ok().json(
            Response::success(json!({"token": token,"role":"user"}))
        ));  // 返回 JWT
    }

    // 如果认证失败，返回 Unauthorized
    Ok(HttpResponse::Unauthorized().finish())
}