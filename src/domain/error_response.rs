use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    message: String,
}

impl ErrorResponse {
    pub fn with_message(message: String) -> Self {
        ErrorResponse { message }
    }
}
