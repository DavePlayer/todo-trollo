use super::AuthErrors;
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use std::{error::Error, fmt};

impl fmt::Display for AuthErrors {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::EnvError(_) => {
                fmt.write_str("Server error. not your fault. check server logs for more info")
            }
            Self::NoClaimsProvided(_) => {
                fmt.write_str("Server error. not your fault. check server logs for more info")
            }
            _ => fmt.write_str("error when loading token"),
        }
    }
}

impl Error for AuthErrors {}

impl error::ResponseError for AuthErrors {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthErrors::EnvError(info) => {
                log::error!("error when loading env variable: {}", info);
            }
            AuthErrors::InvalidToken(info) => {
                log::error!("error when loading token: {}", info);
            }
            AuthErrors::NoClaimsProvided(info) => {
                log::error!("claims in request not provided: {}", info);
            }
        }
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AuthErrors::EnvError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthErrors::InvalidToken(_) => StatusCode::FORBIDDEN,
            AuthErrors::NoClaimsProvided(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
