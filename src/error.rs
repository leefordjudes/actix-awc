use std::{error, fmt, result};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ErrorKind {
    Internal,
    InvalidData,
    NotFound,
}

#[derive(Debug, Clone)]
pub struct Error {
    msg: String,
    code: &'static str,
    kind: ErrorKind,
}

impl Error {
    pub fn new(msg: impl Into<String>, kind: impl Into<ErrorKind>) -> Error {
        Error {
            msg: msg.into(),
            code: "NA",
            kind: kind.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Internal => None,
            ErrorKind::InvalidData => None,
            ErrorKind::NotFound => None,

        }
    }
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    msg: String,
    code: String,
}
impl ErrorResponse {
    pub fn new(code: impl Into<String>, msg: impl Into<String>) -> ErrorResponse {
        ErrorResponse {
            code: code.into(),
            msg: msg.into(),
        }
    }
}

impl actix_web::ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.kind {
            ErrorKind::Internal => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorKind::InvalidData => actix_web::http::StatusCode::BAD_REQUEST,
            ErrorKind::NotFound => actix_web::http::StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse::new(self.code.to_string(), self.msg.to_string());
        actix_web::HttpResponse::build(status_code).json(error_response)
    }
}
