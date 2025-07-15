use axum::{Json, extract::State, http::StatusCode};
use kernel::model::auth::event::CreateToken;
use registry::AppRegistry;
use shared::error::AppResult;

use crate::model::auth::{AccessTokenResponse, LoginRequest};

pub async fn login(
    State(registry): State<AppRegistry>,
    Json(request): Json<LoginRequest>,
) -> AppResult<Json<AccessTokenResponse>> {
    let user_id = registry
        .auth_repository()
        .verify_user(&request.email, &request.password)
        .await?;

    let access_token = registry
        .auth_repository()
        .create_token(CreateToken::new(user_id))
        .await?;

    Ok(Json(AccessTokenResponse {
        access_token: access_token.0,
        user_id,
    }))
}

pub async fn logout(State(registry): State<AppRegistry>) -> AppResult<StatusCode> {
    todo!()
}
