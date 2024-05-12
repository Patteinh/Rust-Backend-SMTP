use crate::data::email_data;
use lettre::{
    message::{Mailbox, SinglePart},
    Message,
};
use std::env;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::Arc;

pub fn load_email_template() -> Arc<String> {
    let path = "./src/templates/email_template.txt";
    if !Path::new(path).exists() {
        panic!("Email template file not found at path: {}", path);
    }
    Arc::new(read_to_string(path).expect("Failed to read email template"))
}

pub fn format_email_template(template: &str, data: &email_data::EmailData) -> String {
    template
        .replace("${purpose}", &data.purpose)
        .replace("${name}", &data.name)
        .replace("${email}", &data.email)
        .replace("${message}", &data.message.replace("\n", "<br>"))
}

pub fn build_email_template(sender_email: &str, formatted_email: &str, purpose: &str) -> Message {
    Message::builder()
        .from(sender_email.parse::<Mailbox>().unwrap())
        .to(env::var("EMAIL_USER").unwrap().parse::<Mailbox>().unwrap())
        .subject(format!("📷 [WEBSITE][{}]", purpose))
        .singlepart(SinglePart::html(formatted_email.to_string()))
        .unwrap()
}
