use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    pub code: u16,
    pub message: String
}

impl ErrorResponse {

    pub fn from_error<T: ResponseError>(err: &T) -> Self {
        Self {
            code: err.status_code().as_u16(),
            message: format!("{}", err),
        }
    }

    pub fn build(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::from_u16(self.code).unwrap()).json(self)
    }

}

#[derive(Debug)]
pub enum HeaderError {
    Missing(String),
    InvalidString(String),
    Parse(String),
}

impl fmt::Display for HeaderError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Missing(hdr) => write!(f, "Missing header: {}", hdr),
            Self::InvalidString(hdr) => write!(f, "Header contains invalid UTF-8 bytes: {}", hdr),
            Self::Parse(hdr) => write!(f, "Cannot parse header: {}", hdr),
        }
    }

}

impl ResponseError for HeaderError {

    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        ErrorResponse::from_error(self).build()
    }

}
