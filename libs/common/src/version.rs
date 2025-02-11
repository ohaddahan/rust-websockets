use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn version() -> impl IntoResponse {
    (StatusCode::OK, env!("CARGO_PKG_VERSION"))
}
