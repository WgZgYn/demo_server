pub mod root {
    use crate::db::DataBase;
    use crate::dto::entity::simple::DeviceAdd;
    use crate::security::auth::Claims;
    use crate::utils;
    use crate::utils::Response;
    use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
    use log::error;

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
            efuse_mac,
            device_name,
            area_id,
            model_id,
        } = data.into_inner();
        match session
            .add_device(&device_name, &efuse_mac, area_id, claims.id(), model_id)
            .await
        {
            Ok(id) => HttpResponse::Ok().json(Response::success(id)),
            Err(e) => {
                error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    pub mod id {
        use crate::db::DataBase;
        use crate::dto::http::request::DeviceUpdate;
        use crate::utils;
        use crate::utils::Response;
        use actix_web::{web, HttpResponse};
        use log::error;

        pub async fn get_device_info(
            path: web::Path<i32>,
            db: web::Data<DataBase>,
        ) -> HttpResponse {
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
            id: web::Path<i32>,
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

            match session
                .update_device_info(id.into_inner(), data.into_inner())
                .await
            {
                Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
                Err(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
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
                Err(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        pub mod status {
            use crate::db::Memory;
            use crate::utils::Response;
            use actix_web::{web, HttpResponse};
            use log::{error, info};

            pub async fn get_device_status(
                path: web::Path<i32>,
                memory: web::Data<Memory>,
            ) -> HttpResponse {
                info!("get_device_status {}", path.clone());
                match memory.get_device_status(path.into_inner()).await {
                    Ok(v) => HttpResponse::Ok().json(Response::success(v)),
                    Err(e) => {
                        error!("{}", e);
                        HttpResponse::InternalServerError().finish()
                    }
                }
            }
        }

        pub mod service {
            use crate::db::DataBase;
            use crate::dto::mqtt::HostToDeviceMessage;
            use crate::service::send_host_message;
            use crate::utils;
            use actix_web::web::Payload;
            use actix_web::{web, HttpResponse};
            use log::{error, info};
            use rumqttc::AsyncClient;

            pub async fn execute_device_service(
                service: web::Path<(i32, String)>,
                mqtt: web::Data<AsyncClient>,
                body: Payload,
                db: web::Data<DataBase>,
            ) -> HttpResponse {
                let (device_id, service_name) = service.into_inner();
                info!("{} {}", device_id, &service_name);

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

                let body = body.to_bytes().await.unwrap_or_default().to_vec();
                let value = serde_json::from_slice(&body).ok();
                let message = HostToDeviceMessage::new(service_name, value);

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
