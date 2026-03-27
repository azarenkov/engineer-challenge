use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct MailerConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from: String,
    pub password_reset_base_url: String,
}

impl MailerConfig {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv::dotenv().ok();
        envy::from_env::<MailerConfig>()
    }
}
