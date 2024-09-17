use axum::response::IntoResponse;

pub async fn signin_handler() -> impl IntoResponse {
    "sigin"
}

pub async fn signup_handler() -> impl IntoResponse {
    "sigup"
}
