use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestPasswordResetRequest {
    #[validate(email)]
    pub email: String,
}

impl From<RequestPasswordResetRequest>
    for application::dto::requests::auth::request_password_reset::RequestPasswordResetDto
{
    fn from(value: RequestPasswordResetRequest) -> Self {
        Self { email: value.email }
    }
}
