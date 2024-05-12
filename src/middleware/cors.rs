use actix_cors::Cors;
use actix_web::http::header;

pub fn cors_setup() -> Cors {
    Cors::default()
        .allowed_origin_fn(|origin, _req_head| {
            origin
                .as_bytes()
                .ends_with(std::env::var("FRONTEND_URL").unwrap().as_bytes())
        })
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::AUTHORIZATION,
        ])
        .supports_credentials()
        .max_age(3600)
}
