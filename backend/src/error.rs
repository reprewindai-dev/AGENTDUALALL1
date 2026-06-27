use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::error;

/// AppError acts as an adapter that bridges robust standard `anyhow::Error`
/// contexts into precise client-facing Axum HTTP Responses. This preserves the 
/// full error chain and backtrace for internal server tracking, while outputting 
/// secure, informative JSON messages for external verification.
pub struct AppError(pub anyhow::Error);

// Allows using the `?` operator over `anyhow::Error` to instantly return an AppError inside Axum handlers.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let err = self.0;
        
        // Log the full diagnostic chain securely on the server side
        error!(
            error_msg = %err,
            error_debug = ?err,
            "Internal system error intercepted"
        );

        // Customize status codes based on common database/validation conditions
        let status = if err.downcast_ref::<sqlx::Error>().is_some() {
            StatusCode::SERVICE_UNAVAILABLE
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };

        let body = Json(json!({
            "success": false,
            "error": err.to_string(),
            "support": "Veklom Mainnet Protocol Team"
        }));

        (status, body).into_response()
    }
}
