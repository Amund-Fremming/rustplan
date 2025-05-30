use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error("Endpoint failed with code {0} error: {1}")]
    HandlerError(StatusCode, String),

    #[error("Critical error: {0}")]
    CriticalError(String),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ServerError::SqlxError(error) => {
                error!("Sqlx error: {:?}", error);
                (StatusCode::INTERNAL_SERVER_ERROR, String::new())
            }
            ServerError::HandlerError(code, error_message) => {
                error!("{}", error_message);
                (code, String::from("Endpoint error"))
            }
            ServerError::CriticalError(error_message) => {
                error!("Critical error: {}", error_message);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    String::from("Critical error."),
                )
            }
        }
        .into_response()
    }
}
