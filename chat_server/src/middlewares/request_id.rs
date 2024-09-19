use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use tracing::warn;

const REQUEST_ID_HEADERS: &'static str = "x-request-id";

pub async fn set_request_id(mut req: Request, next: Next) -> Response {
    let request_id = uuid::Uuid::now_v7().to_string();
    let value = request_id.parse::<HeaderValue>().unwrap();

    let id = req
        .headers_mut()
        .entry(REQUEST_ID_HEADERS)
        .or_insert(value)
        .as_bytes()
        .to_vec();

    let mut res = next.run(req).await;
    match HeaderValue::from_bytes(&id) {
        Ok(v) => {
            res.headers_mut().insert(REQUEST_ID_HEADERS, v);
        }
        Err(e) => {
            warn!("set request id for response failed: {}", e);
        }
    }
    res
}
