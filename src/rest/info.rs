use actix_web::{web, get, Responder};

#[get("/version")]
async fn version() -> impl Responder {
    // Will replace with the crate version on build
    env!("CARGO_PKG_VERSION")
}

pub(super) fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(version);
}
