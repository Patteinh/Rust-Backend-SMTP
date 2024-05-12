use actix_web::{web, App, HttpServer};
use std::env;
use std::io;
use tokio::sync::oneshot;

mod config;
mod controllers;
mod data;
mod middleware;
mod routes;
mod templates;

async fn configure_and_run_server(
    email_template: std::sync::Arc<String>,
    smtp_transport: std::sync::Arc<lettre::SmtpTransport>,
    server_address: String,
) -> io::Result<()> {
    let (tx, rx) = oneshot::channel();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::logger::Logger)
            .app_data(web::Data::new(data::app_state::AppState {
                email_template: email_template.clone(),
                smtp_transport: smtp_transport.clone(),
            }))
            .wrap(middleware::cors::cors_setup())
            .configure(routes::greet::configure)
            .configure(routes::send_email::configure)
    })
    .bind(server_address)?
    .run();

    tokio::spawn(async move {
        let _ = rx.await;
        println!("Server is running!");
    });

    tx.send(()).ok();

    server.await
}

fn get_server_addresses() -> (String, String) {
    let server_address =
        env::var("BACKEND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let server_address_local =
        env::var("BACKEND_ADDRESS_LOCAL").unwrap_or_else(|_| "localhost:8080".to_string());
    (server_address, server_address_local)
}

fn announce_server_start(server_address: &str, server_address_local: &str) {
    println!(
        "Starting server at http://{} and http://{}",
        server_address, server_address_local
    );
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let email_template = templates::email::load_email_template();
    let smtp_transport = config::smtp::configure_smtp_transport();

    let (server_address, server_address_local) = get_server_addresses();

    announce_server_start(&server_address, &server_address_local);
    configure_and_run_server(email_template, smtp_transport, server_address).await?;
    Ok(())
}
