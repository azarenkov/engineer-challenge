use std::{error::Error, sync::Arc};

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};

use application::{command::auth::register::RegisterUser, query::auth::login::LoginUser};
use infrastructure::{
    config::postgres::PostgresConfig,
    database::postgres::{
        connection::create_postgres_pool, repositories::user::PostgresUserRepository,
    },
    hasher::bcrypt::BcryptHasher,
};
use shared::rate_limiting::{
    config::RateLimitingConfig, create_governor_config, create_rate_limiting,
};

use crate::{config::ServerConfig, routes, shared::state::AppState};

pub async fn run() -> Result<(), Box<dyn Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let server_config = ServerConfig::from_env()?;
    let postgres_config = PostgresConfig::from_env()?;
    let rate_limiting_config = RateLimitingConfig::from_env()?;

    let postgres_pool = create_postgres_pool(postgres_config).await?;

    let user_repository = Arc::new(PostgresUserRepository::new(postgres_pool.clone()));

    let hasher = Arc::new(BcryptHasher::default());

    let register_user = RegisterUser::new(user_repository.clone(), hasher.clone());
    let login_user = LoginUser::new(user_repository.clone(), hasher.clone());

    let app_state = web::Data::new(AppState::new(
        register_user,
        login_user,
        server_config.jwt_secret,
    ));

    let governor_config = create_governor_config(rate_limiting_config)
        .ok_or("Failed to create rate limiting configuration: invalid parameters")?;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let rate_limiting = create_rate_limiting(&governor_config);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(rate_limiting)
            .configure(routes::configure)
    })
    .bind((server_config.host, server_config.port))?
    .run()
    .await?;

    Ok(())
}
