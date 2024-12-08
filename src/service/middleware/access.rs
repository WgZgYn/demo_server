use crate::db::DataBase;
use crate::security::auth::get_id_from_http_request;
use actix_web::dev::forward_ready;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, FromRequest,
};
use futures_util::future::LocalBoxFuture;
use log::info;
use std::future::Ready;

pub struct AccessAuth;

#[derive(Clone)]
pub struct AccessAuthMiddleware<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for AccessAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AccessAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(AccessAuthMiddleware { service }))
    }
}

impl<S, B> Service<ServiceRequest> for AccessAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let rq = req.request();
        let account_id = get_id_from_http_request(rq).ok_or("no account id");
        let db = web::Data::<DataBase>::extract(rq).into_inner();
        let house_id = web::Path::<i32>::extract(rq).into_inner();

        if account_id.is_err() {
            return Box::pin(async { Err(actix_web::error::ErrorUnauthorized("no account_id")) });
        }

        if house_id.is_err() {
            return Box::pin(async { Err(actix_web::error::ErrorBadRequest("no house_id")) });
        }

        if db.is_err() {
            return Box::pin(async { Err(actix_web::error::ErrorInternalServerError("db error")) });
        }

        let account_id = account_id.unwrap();
        let db = db.unwrap();
        let house_id = house_id.unwrap();
        let auth = check_access(db, account_id, house_id.into_inner());
        let fut = self.service.call(req);
        Box::pin(async move {
            if let Ok(true) = auth.await {
                info!("AccessAuth Check Ok");
                let res = fut.await?;
                Ok(res)
            } else {
                info!("AccessAuth Check Failed");
                Err(actix_web::error::ErrorUnauthorized("Access denied"))
            }
        })
    }
}
async fn check_access(
    db: web::Data<DataBase>,
    account_id: i32,
    house_id: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let session = db.get_session().await?;
    Ok(session.is_member(account_id, house_id).await)
}
