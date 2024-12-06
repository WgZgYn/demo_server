pub mod root {
    use crate::db::DataBase;
    use crate::dto::http::request::HouseAdd;
    use crate::security::auth::Claims;
    use crate::utils;
    use crate::utils::Response;
    use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
    use log::error;

    pub async fn get_all_house_info(req: HttpRequest, db: web::Data<DataBase>) -> HttpResponse {
        let e = req.extensions();
        let claims = match e.get::<Claims>() {
            Some(claims) => claims,
            None => return HttpResponse::Unauthorized().finish(),
        };
        match db.get_session().await {
            Ok(session) => match session.get_all_house_info(claims.id()).await {
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

    // TODO: test
    pub async fn add_house(
        data: web::Json<HouseAdd>,
        req: HttpRequest,
        db: web::Data<DataBase>,
    ) -> HttpResponse {
        let e = req.extensions();
        let claims = match e.get::<Claims>() {
            Some(claims) => claims,
            None => return HttpResponse::Unauthorized().finish(),
        };
        let account_id = claims.id();

        let mut session = match db.get_session().await {
            Ok(session) => session,
            Err(e) => {
                error!("{e}");
                return HttpResponse::InternalServerError().finish();
            }
        };

        match session.add_house(&data.house_name, account_id).await {
            Ok(id) => HttpResponse::Ok().json(Response::success(id)),
            Err(e) => {
                error!("{}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    pub mod id {
        use crate::db::DataBase;
        use crate::dto::entity::simple::HouseInfo;
        use crate::dto::http::request::HouseUpdate;
        use crate::utils;
        use crate::utils::Response;
        use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
        use log::error;
        use std::error::Error;
        use std::future::Future;
        use crate::security::auth::{get_id_from_http_request, Claims};

        pub async fn get_house_info(
            data: web::Path<i32>,
            db: web::Data<DataBase>,
            req: HttpRequest,
        ) -> HttpResponse {
            let session = match db.get_session().await {
                Ok(session) => session,
                Err(e) => {
                    error!("{}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            };
            match session.get_house_info(data.into_inner()).await {
                Ok(v) => HttpResponse::Ok().json(Response::success(v)),
                Err(e) => {
                    error!("{}", e);
                    HttpResponse::NotFound().finish()
                }
            }
        }

        pub async fn update_house_info(
            id: web::Path<i32>,
            data: web::Json<HouseUpdate>,
            db: web::Data<DataBase>,
            req: HttpRequest,
        ) -> HttpResponse {
            let session = match db.get_session().await {
                Ok(session) => session,
                Err(e) => {
                    error!("{}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            };

            match session
                .update_house_info(id.into_inner(), data.into_inner())
                .await
            {
                Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
                Err(e) => {
                    error!("{}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }

        pub async fn delete_house(req: HttpRequest, house_id: web::Path<i32>, db: web::Data<DataBase>) -> HttpResponse {
            let account_id = match get_id_from_http_request(&req) {
                Some(id) => id,
                None => return HttpResponse::Unauthorized().finish(),
            };

            let session = match db.get_session().await {
                Ok(session) => session,
                Err(e) => {
                    error!("{}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            };

            match session.is_house_created_by(house_id.clone(), account_id).await {
                Ok(true) => {
                    match session.delete_house(house_id.into_inner()).await {
                        Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
                        Err(e) => {
                            error!("{}", e);
                            HttpResponse::InternalServerError().finish()
                        }
                    }
                }
                Ok(false) => {
                    match session.delete_member(house_id.into_inner(), account_id).await {
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
    }
}
