use crate::controllers::greet::greet;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(greet));
    cfg.route("/", web::head().to(greet));
}
