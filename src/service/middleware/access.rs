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

pub struct HouseAccessAuth;

pub struct HouseAccessAuthMiddleware<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for HouseAccessAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = HouseAccessAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(HouseAccessAuthMiddleware { service }))
    }
}

impl<S, B> Service<ServiceRequest> for HouseAccessAuthMiddleware<S>
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
        let auth = check_house_access(db, account_id, house_id.into_inner());
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

pub struct AreaAccessAuth;

pub struct AreaAccessAuthMiddleware<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for AreaAccessAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AreaAccessAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(AreaAccessAuthMiddleware { service }))
    }
}

impl<S, B> Service<ServiceRequest> for AreaAccessAuthMiddleware<S>
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
        let area_id = web::Path::<i32>::extract(rq).into_inner();

        if account_id.is_err() {
            return Box::pin(async { Err(actix_web::error::ErrorUnauthorized("no account_id")) });
        }

        if area_id.is_err() {
            return Box::pin(async { Err(actix_web::error::ErrorBadRequest("no house_id")) });
        }

        if db.is_err() {
            return Box::pin(async { Err(actix_web::error::ErrorInternalServerError("db error")) });
        }

        let account_id = account_id.unwrap();
        let db = db.unwrap();
        let area_id = area_id.unwrap();
        let auth = check_area_access(db, account_id, area_id.into_inner());
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

pub struct DeviceAccessAuth;

pub struct DeviceAccessAuthMiddleware<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for DeviceAccessAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = DeviceAccessAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(DeviceAccessAuthMiddleware { service }))
    }
}

impl<S, B> Service<ServiceRequest> for DeviceAccessAuthMiddleware<S>
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
        let device_id = web::Path::<i32>::extract(rq).into_inner();

        if account_id.is_err() {
            return Box::pin(async { Err(actix_web::error::ErrorUnauthorized("no account_id")) });
        }

        if device_id.is_err() {
            return Box::pin(async { Err(actix_web::error::ErrorBadRequest("no device_id")) });
        }

        if db.is_err() {
            return Box::pin(async { Err(actix_web::error::ErrorInternalServerError("db error")) });
        }

        let account_id = account_id.unwrap();
        let db = db.unwrap();
        let device_id = device_id.unwrap();
        let auth = check_device_access(db, account_id, device_id.into_inner());
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

async fn check_house_access(
    db: web::Data<DataBase>,
    account_id: i32,
    house_id: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let session = db.get_session().await?;
    Ok(session.is_member(account_id, house_id).await)
}

async fn check_area_access(
    db: web::Data<DataBase>,
    account_id: i32,
    area_id: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let session = db.get_session().await?;
    Ok(session.can_access_area_by(area_id, account_id).await?)
}

async fn check_device_access(
    db: web::Data<DataBase>,
    account_id: i32,
    device_id: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let session = db.get_session().await?;
    Ok(session.can_access_device_by(device_id, account_id).await?)
}
