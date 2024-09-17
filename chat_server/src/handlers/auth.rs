use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    models::{user::CreateUser, User},
    AppError, AppState,
};

pub async fn signin_handler(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = User::create(&input, &state.pool).await?;
    let token = state.ek.sign(user)?;
    Ok((StatusCode::CREATED, token))
}

pub async fn signup_handler() -> impl IntoResponse {
    "sigup"
}
