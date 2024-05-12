use crate::controllers::send_email::send_email;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/send-email").route(web::post().to(send_email)));
}
