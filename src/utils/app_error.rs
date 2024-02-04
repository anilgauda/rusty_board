use axum::http::StatusCode;

pub struct AppError {
    code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into <String>) -> Self{
        Self {
            code,
            message: message.into()
        }
    }
}