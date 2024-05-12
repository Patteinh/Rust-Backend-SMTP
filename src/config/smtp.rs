use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;
use std::env;
use std::sync::Arc;

pub fn configure_smtp_transport() -> Arc<SmtpTransport> {
    Arc::new(
        SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(Credentials::new(
                env::var("EMAIL_USER").unwrap(),
                env::var("EMAIL_PASSWORD").unwrap(),
            ))
            .build(),
    )
}
