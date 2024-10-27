// Make our own error that wraps `anyhow::Error`.
pub struct AppError(anyhow::Error);
pub type AppResult<T> = Result<T, AppError>;

// Tell axum how to convert `AppError` into a response.
impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        log::error!("Server error: {}", self.0);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            #[cfg(debug_assertions)]
            format!("500 Server Error: {:?}", self.0),
            #[cfg(not(debug_assertions))]
            format!("500 Server Error"),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
