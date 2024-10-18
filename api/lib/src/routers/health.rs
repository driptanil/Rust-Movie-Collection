use actix_web::web;

use crate::services::health::get_health;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(get_health));
}
