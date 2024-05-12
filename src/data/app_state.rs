use lettre::SmtpTransport;
use std::sync::Arc;
pub struct AppState {
    pub email_template: Arc<String>,
    pub smtp_transport: Arc<SmtpTransport>,
}
