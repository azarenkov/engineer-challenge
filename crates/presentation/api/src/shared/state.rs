use application::{command::auth::register::RegisterUser, query::auth::login::LoginUser};
use infrastructure::{
    database::postgres::repositories::user::PostgresUserRepository, hasher::bcrypt::BcryptHasher,
};

pub struct AppState {
    pub register_user: RegisterUser<PostgresUserRepository, BcryptHasher>,
    pub login_user: LoginUser<PostgresUserRepository, BcryptHasher>,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(
        register_user: RegisterUser<PostgresUserRepository, BcryptHasher>,
        login_user: LoginUser<PostgresUserRepository, BcryptHasher>,
        jwt_secret: String,
    ) -> Self {
        Self {
            register_user,
            login_user,
            jwt_secret,
        }
    }
}
