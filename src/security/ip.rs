use std::cell::RefCell;
use std::collections::HashSet;
use std::future::{ready, Ready};
use std::net::IpAddr;
use std::sync::{Arc, LockResult};
use std::task::{Context, Poll};
use std::time::Instant;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpResponse};
use futures_util::future::LocalBoxFuture;
use log::{debug, info};
use std::sync::RwLock;
use actix_web::error::ErrorForbidden;

#[derive(Default)]
pub struct RecordIP {
    whitelist: Arc<RwLock<HashSet<IpAddr>>>,
    blacklist: Arc<RwLock<HashSet<IpAddr>>>,
    record: Arc<RwLock<HashSet<IpAddr>>>,
}

impl<S, B> Transform<S, ServiceRequest> for RecordIP
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RecordIPMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RecordIPMiddleware {
            service,
            whitelist: self.whitelist.clone(),
            blacklist: self.blacklist.clone(),
            record: self.record.clone(),
        }))
    }
}

pub struct RecordIPMiddleware<S> {
    service: S,
    whitelist: Arc<RwLock<HashSet<IpAddr>>>,
    blacklist: Arc<RwLock<HashSet<IpAddr>>>,
    record: Arc<RwLock<HashSet<IpAddr>>>,
}

impl<S, B> Service<ServiceRequest> for RecordIPMiddleware<S>
where
    S: Service<ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let ip = req.peer_addr().map(|addr| addr.ip());
        if let Some(ip) = ip {
            info!("ip: {ip}, visit the website");
            let g = self.record.write();
            match g {
                Ok(mut set) => {
                    set.insert(ip);
                }
                Err(_) => {
                    debug!("rwlock")
                }
            }

            match self.whitelist.read() {
                Ok(set) => {
                    if set.contains(&ip) {
                        return Box::pin(async { Err(actix_web::Error::from(ErrorForbidden("IP was baned"))) });
                    }
                }
                Err(_) => {}
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}