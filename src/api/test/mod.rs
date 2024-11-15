mod ping;
mod test_auth;

use crate::api::auth::Auth;
use crate::utils;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse};
pub use ping::ping;
use rumqttc::AsyncClient;
pub use test_auth::get_auth_info;

async fn test_mqtt(client: web::Data<AsyncClient>) -> HttpResponse {
    println!("test_mqtt client: {:?}", client);
    HttpResponse::Ok().json(utils::Result::success())
}

pub fn config_test(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/test")
            .service(web::resource("/ping").route(web::get().to(ping)))
            .service(web::resource("/mqtt").route(web::get().to(test_mqtt)))
            .service(
                web::scope("/auth")
                    .wrap(Auth)
                    .route("", web::get().to(get_auth_info)),
            ),
    );
}
