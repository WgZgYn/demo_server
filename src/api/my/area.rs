pub mod root {
    use crate::db::DataBase;
    use crate::dto::http::request::AreaAdd;
    use crate::security::auth::Claims;
    use crate::utils;
    use crate::utils::Response;
    use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
    use log::error;

    pub async fn get_all_area_info(req: HttpRequest, db: web::Data<DataBase>) -> HttpResponse {
        let e = req.extensions();
        let claims = match e.get::<Claims>() {
            Some(claims) => claims,
            None => return HttpResponse::Unauthorized().finish(),
        };

        match db.get_session().await {
            Ok(session) => match session.get_all_area_info(claims.id()).await {
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

    pub async fn add_area(
        data: web::Json<AreaAdd>,
        req: HttpRequest,
        db: web::Data<DataBase>,
    ) -> HttpResponse {
        let e = req.extensions();
        let claims = match e.get::<Claims>() {
            Some(claims) => claims,
            None => return HttpResponse::Unauthorized().finish(),
        };
        let account_id = claims.id();

        let session = match db.get_session().await {
            Ok(session) => session,
            Err(e) => {
                error!("{e}");
                return HttpResponse::InternalServerError().finish();
            }
        };

        match session
            .add_area(&data.area_name, data.house_id, account_id)
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
        use crate::dto::http::request::AreaUpdate;
        use crate::utils::Response;
        use actix_web::{web, HttpRequest, HttpResponse};
        use deadpool_postgres::PoolError;
        use log::error;
        use crate::utils;

        pub async fn get_area_info(
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
            match session.get_area_info(data.into_inner()).await {
                Ok(v) => HttpResponse::Ok().json(Response::success(v)),
                Err(e) => {
                    error!("{}", e);
                    HttpResponse::NotFound().finish()
                }
            }
        }
        pub async fn update_area_info(
            data: web::Json<AreaUpdate>,
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

            match session.update_area_info(data.into_inner()).await {
                Ok(_) => HttpResponse::Ok().json(utils::Result::success()),
                Err(e) => { error!("{}", e); HttpResponse::InternalServerError().finish() }
            }
        }
        pub async fn delete_area() -> HttpResponse {
            // TODO:
            HttpResponse::NotFound().finish()
        }
    }
}
