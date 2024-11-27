pub mod root {
    use crate::db::{DataBase, Memory};
    use crate::dto::entity::simple::{DeviceAdd, DeviceInfo};
    use crate::dto::mqtt::HostMessage;
    use crate::security::auth::Claims;
    use crate::service::send_host_message;
    use crate::utils;
    use crate::utils::Response;
    use actix_web::http::header::CONTENT_TYPE;
    use actix_web::http::Method;
    use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
    use log::error;
    use rumqttc::AsyncClient;

    pub async fn get_all_devices(req: HttpRequest, db: web::Data<DataBase>) -> HttpResponse {
        let e = req.extensions();
        let claims = match e.get::<Claims>() {
            Some(claims) => claims,
            None => return HttpResponse::Unauthorized().finish(),
        };

        let session = match db.get_session().await {
            Ok(session) => session,
            Err(e) => {
                error!("{}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };

        match session
            .get_account_devices(claims.id(), claims.sub().to_string())
            .await
        {
            Ok(v) => HttpResponse::Ok().json(Response::success(v)),
            Err(e) => {
                error!("{}", e);
                return HttpResponse::NotFound().json(utils::Result::error("No such Account"));
            }
        }
    }
    pub async fn add_device(
        data: web::Json<DeviceAdd>,
        db: web::Data<DataBase>,
        req: HttpRequest,
    ) -> HttpResponse {
        let e = req.extensions();
        let claims = match e.get::<Claims>() {
            Some(claims) => claims,
            None => return HttpResponse::Unauthorized().finish(),
        };

        let session = match db.get_session().await {
            Ok(session) => session,
            Err(e) => {
                error!("{}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };

        let DeviceAdd {
            device_mac,
            device_name,
            area_id,
            model_id,
        } = data.into_inner();
        match session
            .add_device(&device_name, &device_mac, area_id, claims.id(), model_id)
            .await
        {
            Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
            Err(e) => {
                error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    pub mod id {
        use crate::db::DataBase;
        use crate::dto::http::request::DeviceUpdate;
        use crate::security::auth::Claims;
        use crate::utils::Response;
        use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
        use log::error;
        use crate::utils;

        pub async fn get_device_info(
            req: HttpRequest,
            path: web::Path<i32>,
            db: web::Data<DataBase>,
        ) -> HttpResponse {
            let e = req.extensions();
            let claims = match e.get::<Claims>() {
                Some(claims) => claims,
                None => return HttpResponse::Unauthorized().finish(),
            };

            match db.get_session().await {
                Ok(session) => match session.get_device_info(*path).await {
                    Ok(v) => HttpResponse::Ok().json(Response::success(v)),
                    Err(e) => HttpResponse::NotFound().json(e.to_string()),
                },
                Err(e) => {
                    error!("{e}");
                    HttpResponse::InternalServerError().finish()
                }
            }
        }

        pub async fn update_device_info(
            data: web::Json<DeviceUpdate>,
            db: web::Data<DataBase>,
        ) -> HttpResponse {
            let session = match db.get_session().await {
                Ok(session) => session,
                Err(e) => {
                    error!("{}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            };

            match session.update_device_info(data.into_inner()).await {
                Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
                Err(e) => { error!("{}", e); HttpResponse::InternalServerError().finish() }
            }
        }

        pub async fn delete_device(data: web::Path<i32>, db: web::Data<DataBase>) -> HttpResponse {
            let session = match db.get_session().await {
                Ok(session) => session,
                Err(e) => {
                    error!("{}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            };

            match session.delete_device(data.into_inner()).await {
                Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
                Err(e) => { error!("{}", e); HttpResponse::InternalServerError().finish() }
            }
        }
        pub mod status {
            use crate::db::Memory;
            use crate::utils::Response;
            use actix_web::{web, HttpResponse};

            pub async fn get_device_status(
                path: web::Path<i32>,
                memory: web::Data<Memory>,
            ) -> HttpResponse {
                let guard = memory.device_state.read().await;
                match guard.status(path.into_inner()) {
                    Some(v) => HttpResponse::Ok().json(Response::success(v)),
                    None => HttpResponse::NotFound().finish(),
                }
            }
        }

        pub mod service {
            use crate::db::DataBase;
            use crate::dto::mqtt::HostMessage;
            use crate::security::auth::Claims;
            use crate::service::send_host_message;
            use crate::utils;
            use actix_web::http::header::CONTENT_TYPE;
            use actix_web::http::Method;
            use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
            use actix_web::web::Payload;
            use log::error;
            use rumqttc::AsyncClient;

            pub async fn execute_device_service(
                req: HttpRequest,
                service: web::Path<(i32, String)>,
                mqtt: web::Data<AsyncClient>,
                body: Payload,
                db: web::Data<DataBase>,
            ) -> HttpResponse {
                let e = req.extensions();
                let claims = match e.get::<Claims>() {
                    Some(claims) => claims,
                    None => return HttpResponse::Unauthorized().finish(),
                };

                let (device_id, service_name) = service.into_inner();
                println!("{device_id} {service_name}");

                let session = match db.get_session().await {
                    Ok(session) => session,
                    Err(e) => {
                        error!("{}", e);
                        return HttpResponse::InternalServerError().finish();
                    }
                };

                let mac = match session.get_device_mac_by_id(device_id).await {
                    Ok(mac) => mac,
                    Err(e) => {
                        error!("{e}");
                        return HttpResponse::NotFound()
                            .json(utils::Result::error("no such device"));
                    }
                };

                let content_type = req
                    .headers()
                    .get(CONTENT_TYPE)
                    .and_then(|ct| ct.to_str().ok());
                let body = body.to_bytes().await.unwrap_or_default().to_vec();
                let message = match *req.method() {
                    Method::POST => match content_type {
                        Some("application/json") => HostMessage::json(service_name, body),
                            Some("text/plain") => HostMessage::text(service_name, body),
                        _ => {
                            return HttpResponse::BadRequest()
                                .json(utils::Result::error("unsupported content-type"));
                        }
                    },
                    Method::GET => HostMessage::none(service_name),
                    _ => {
                        return HttpResponse::BadRequest()
                            .json(utils::Result::error("unsupported method"))
                    }
                };
                match send_host_message(mqtt, &mac, message).await {
                    Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
                    Err(e) => {
                        error!("{e}");
                        HttpResponse::InternalServerError().finish()
                    }
                }
            }
        }
    }
}
