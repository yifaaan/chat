mod config;
mod error;
mod handlers;
mod models;
mod utils;

use axum::{
    routing::{get, patch, post},
    Router,
};
pub use config::AppConfig;
use core::fmt;
pub use error::AppError;
use handlers::*;
use std::{ops::Deref, sync::Arc};
use utils::{DecodingKey, EncodingKey};

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    inner: Arc<AppStateInner>,
}

pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
    pub(crate) ek: EncodingKey,
    pub(crate) dk: DecodingKey,
}

pub fn get_router(config: AppConfig) -> Router {
    let state = AppState::new(config);

    let api = Router::new()
        .route("/signin", post(signin_handler))
        .route("/signup", post(signup_handler))
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat/:id",
            patch(update_chat_handler)
                .delete(delete_chat_handler)
                .post(send_message_handler),
        )
        .route("/chat/:id/messages", get(list_message_handler));

    let app = Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state);
    app
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        // 加载私钥
        let ek = EncodingKey::load(&config.auth.sk).expect("load secret key failed");
        // 加载公钥
        let dk = DecodingKey::load(&config.auth.pk).expect("load public key failed");
        Self {
            inner: Arc::new(AppStateInner { config, ek, dk }),
        }
    }
}

impl Deref for AppState {
    type Target = Arc<AppStateInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl fmt::Debug for AppStateInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppStateInner")
            .field("config", &self.config)
            .finish()
    }
}
