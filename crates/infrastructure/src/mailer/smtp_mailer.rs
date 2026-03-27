use async_trait::async_trait;
use lettre::Message;
use lettre::message::header::ContentType;
use lettre::transport::smtp::{AsyncSmtpTransport, Tokio1Executor, authentication::Credentials};
use std::sync::Arc;

use crate::config::mailer::MailerConfig;
use application::ports::mailer::{Mailer, MailerError};
use urlencoding::encode;

#[derive(Clone)]
pub struct SmtpMailer {
    inner: Arc<Inner>,
}

struct Inner {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from: String,
    reset_base_url: String,
}

impl SmtpMailer {
    pub fn from_config(cfg: MailerConfig) -> Result<Self, MailerError> {
        let host = cfg.host;
        let port = cfg.port;
        let username = cfg.username;
        let password = cfg.password;
        let from = cfg.from;
        let reset_base_url = cfg.password_reset_base_url;

        let builder = AsyncSmtpTransport::<Tokio1Executor>::relay(&host).map_err(|e| {
            MailerError::Sending(format!("failed to create relay for host {}: {}", host, e))
        })?;

        let creds = Credentials::new(username, password);

        let transport = builder.credentials(creds).port(port).build();

        let inner = Inner {
            mailer: transport,
            from,
            reset_base_url,
        };

        Ok(Self {
            inner: Arc::new(inner),
        })
    }
}

#[async_trait]
impl Mailer for SmtpMailer {
    async fn send_password_reset(&self, to: &str, token: &str) -> Result<(), MailerError> {
        let reset_link = format!(
            "{}/reset-password?token={}",
            self.inner.reset_base_url.trim_end_matches('/'),
            encode(token)
        );

        let html_body = format!(
            r#"
            <p>Здравствуйте,</p>
            <p>Вы запросили сброс пароля. Перейдите по ссылке ниже, чтобы задать новый пароль:</p>
            <p><a href="{link}">{link}</a></p>
            <p>Если вы не запрашивали сброс пароля, просто проигнорируйте это сообщение.</p>
            "#,
            link = reset_link
        );

        let email =
            Message::builder()
                .from(self.inner.from.parse().map_err(|e| {
                    MailerError::Sending(format!("invalid MAILER_FROM (from): {}", e))
                })?)
                .to(to.parse().map_err(|e| {
                    MailerError::Sending(format!("invalid recipient address: {}", e))
                })?)
                .subject("Сброс пароля")
                .header(ContentType::TEXT_HTML)
                .body(html_body)
                .map_err(|e| MailerError::Sending(format!("failed to build message: {}", e)))?;

        self.inner
            .mailer
            .send(email)
            .await
            .map_err(|e| MailerError::Sending(format!("failed to send email: {}", e)))?;

        Ok(())
    }
}
