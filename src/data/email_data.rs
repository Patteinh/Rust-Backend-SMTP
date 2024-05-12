use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EmailData {
    pub name: String,
    pub email: String,
    pub purpose: String,
    pub message: String,
}
