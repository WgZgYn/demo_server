use actix_files::Files;
use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse};
use log::info;

pub fn vue() -> Files {
    Files::new("/", "./html").index_file("index.html")
}
pub fn config_web(cfg: &mut ServiceConfig) {
    cfg.service(vue());
}
pub fn config_redirects(cfg: &mut ServiceConfig) {
    cfg.default_service(web::get().to(|| {
        info!("redirect to /");
        let mut b = HttpResponse::PermanentRedirect();
        b.append_header(("Location", "/"));
        b
    }));
}
