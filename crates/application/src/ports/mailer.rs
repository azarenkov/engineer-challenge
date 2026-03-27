use async_trait::async_trait;

#[async_trait]
pub trait Mailer {
    async fn send_password_reset(&self, to: &str, token: &str) -> Result<(), MailerError>;
}

#[derive(Debug, thiserror::Error)]
pub enum MailerError {
    #[error("Mailer error: {0}")]
    Sending(String),
}
