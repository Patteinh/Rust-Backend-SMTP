use crate::data;
use crate::templates;
use actix_web::{web, HttpResponse, Responder};
use lettre::Transport;

pub async fn send_email(
    data: web::Json<data::email_data::EmailData>,
    state: web::Data<data::app_state::AppState>,
) -> impl Responder {
    let formatted_email = templates::email::format_email_template(&state.email_template, &data);
    let email =
        templates::email::build_email_template(&data.email, &formatted_email, &data.purpose);

    match state.smtp_transport.send(&email) {
        Ok(_) => HttpResponse::Ok().body("Email sent successfully!"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to send email: {}", e)),
    }
}
