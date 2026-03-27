use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 4, max = 30))]
    pub password: String,
}

impl From<RegisterUserRequest>
    for application::dto::requests::auth::register_user::RegisterUserRequest
{
    fn from(value: RegisterUserRequest) -> Self {
        Self {
            email: value.email,
            password: value.password,
        }
    }
}
