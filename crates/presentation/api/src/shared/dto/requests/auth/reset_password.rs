use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordRequest {
    pub token: String,
    #[validate(length(min = 4, max = 30))]
    pub new_password: String,
}

impl From<ResetPasswordRequest>
    for application::dto::requests::auth::reset_password::ResetPasswordDto
{
    fn from(value: ResetPasswordRequest) -> Self {
        Self {
            token: value.token,
            new_password: value.new_password,
        }
    }
}
