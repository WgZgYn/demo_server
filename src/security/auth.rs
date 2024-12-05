use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::extractors::{bearer, AuthenticationError};
use chrono::{Duration, Utc};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

const SECRET_KEY: &[u8] = b"AAABBBCCC123456";

pub enum Role {
    Admin,
    User,
}

impl Role {
    pub fn as_str(&self) -> &str {
        match self {
            Role::Admin => "admin",
            Role::User => "user",
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    id: i32,
    sub: String,
    role: String,
    exp: usize,
}

impl Claims {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn role(&self) -> &str {
        self.role.as_str()
    }
    pub fn sub(&self) -> &str {
        self.sub.as_str()
    }
}

pub fn create_token(user: String, role: Role, id: i32) -> String {
    let jwt_exp = Utc::now() + Duration::hours(2);

    let claims = Claims {
        id,
        sub: user,
        role: role.as_str().to_string(),
        exp: jwt_exp.timestamp() as usize, // 设置过期时间
    };
    let token = encode(
        &Header::default(),
        &claims,
        // This can be lazy init
        &EncodingKey::from_secret(SECRET_KEY),
    )
        .unwrap();
    token
}

pub fn validate_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    if let Ok(_) = validate_token(credentials.token()) {
        Ok(req)
    } else {
        let config = req
            .app_data::<bearer::Config>()
            .cloned()
            .unwrap_or_default()
            .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");

        Err((AuthenticationError::from(config).into(), req))
    }
}
pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    actix_web::dev::Response<B>: From<HttpResponse>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
    S::Future: 'static,
    actix_web::dev::Response<B>: From<HttpResponse>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 从请求头中提取 JWT 令牌
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "));

        // 验证 JWT
        if let Some(token) = token {
            if let Ok(token_data) = validate_token(token) {
                req.extensions_mut().insert(token_data.claims); // 将 Claims 存储到 req 中
                // 如果 JWT 验证通过，继续处理请求
                let fut = self.service.call(req);
                return Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                });
            }
        }

        Box::pin(async {
            Ok::<ServiceResponse<B>, Error>(
                req.into_response::<B, _>(HttpResponse::Unauthorized().finish()),
            )
        })
    }
}


pub fn get_id_from_http_request(req: &HttpRequest) -> Option<i32> {
    let e = req.extensions();
    e.get::<Claims>().map(|claims| claims.id)
}