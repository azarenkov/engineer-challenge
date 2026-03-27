use actix_web::{HttpResponse, Responder, http::StatusCode, post, web};
use application::query::auth::login::LoginError;
use shared::jwt::validation::{encode_token_pair, validate_refresh_token};
use validator::Validate;

use crate::shared::{
    dto::{
        requests::auth::{
            login::LoginUserRequest, refresh_token::RefreshTokenRequest,
            registration::RegisterUserRequest, request_password_reset::RequestPasswordResetRequest,
            reset_password::ResetPasswordRequest,
        },
        responses::{ApiResponse, auth::registration::RegisterUserResponse},
    },
    error::ApiError,
    state::AppState,
};

#[post("/register")]
async fn register_user(
    app_state: web::Data<AppState>,
    request: web::Json<RegisterUserRequest>,
) -> Result<impl Responder, ApiError> {
    let request: RegisterUserRequest = request.into_inner();
    request.validate()?;

    let user = app_state.register_user.execute(request.into()).await?;

    let response = RegisterUserResponse::from(user);

    Ok(HttpResponse::Created().json(ApiResponse::success(response)))
}

#[post("/login")]
async fn login_user(
    app_state: web::Data<AppState>,
    request: web::Json<LoginUserRequest>,
) -> Result<impl Responder, ApiError> {
    let request: LoginUserRequest = request.into_inner();
    request.validate()?;

    let user_id = app_state.login_user.execute(request.into()).await?;

    let token_pair = encode_token_pair(user_id.uuid(), &app_state.jwt_secret)
        .await
        .map_err(|_| ApiError::from(LoginError::Internal))?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(token_pair)))
}

#[post("/refresh")]
async fn refresh_tokens(
    app_state: web::Data<AppState>,
    request: web::Json<RefreshTokenRequest>,
) -> Result<impl Responder, ApiError> {
    let request: RefreshTokenRequest = request.into_inner();

    let user_uuid = validate_refresh_token(&request.refresh_token, &app_state.jwt_secret)
        .await
        .map_err(|_| {
            ApiError::new(
                "Invalid refresh token".to_string(),
                StatusCode::UNAUTHORIZED,
            )
        })?;

    let token_pair = encode_token_pair(user_uuid, &app_state.jwt_secret)
        .await
        .map_err(|_| {
            ApiError::new(
                "Failed to encode tokens".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(token_pair)))
}

#[post("/request-password-reset")]
async fn request_password_reset(
    app_state: web::Data<AppState>,
    request: web::Json<RequestPasswordResetRequest>,
) -> Result<impl Responder, ApiError> {
    let request: RequestPasswordResetRequest = request.into_inner();
    request.validate()?;

    app_state
        .request_password_reset
        .execute(request.into())
        .await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success("Password reset email sent")))
}

#[post("/reset-password")]
async fn reset_password(
    app_state: web::Data<AppState>,
    request: web::Json<ResetPasswordRequest>,
) -> Result<impl Responder, ApiError> {
    let request: ResetPasswordRequest = request.into_inner();
    request.validate()?;

    let user_id = app_state.reset_password.execute(request.into()).await?;

    Ok(HttpResponse::Ok().json(ApiResponse::success(format!(
        "Password reset successfully for user: {}",
        user_id.uuid()
    ))))
}
