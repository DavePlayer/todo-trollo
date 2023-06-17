use actix_web::web::Json;
use actix_web::{error, http::StatusCode, HttpResponse};
use std::{error::Error, fmt};

use crate::models::{
    group::Grup,
    user::{UserToLogin, UserToRegister},
};

#[derive(Debug)]
pub enum DatabaseErrors {
    SelectError(String),
    CantEstablishConnection(String),
    UserExists(Json<UserToRegister>),
    InsertError(String),
    UpdateError(String),
    UserNotFound(UserToLogin),
    GroupExist(Grup),
    DataNotFound(String, String),
    DataExists(String, String),
    NoSuchUser(String),
    AlreadyInGroup(Vec<i32>),
}
mod database_errors;

#[derive(Debug)]
pub enum AuthErrors {
    InvalidToken(String),
    EnvError(String),
    NoClaimsProvided(String),

    #[allow(dead_code)]
    WebSocketError(String),
}
mod auth_errors;

#[derive(Debug)]
pub enum UltimateError {
    Database(DatabaseErrors),
    Auth(AuthErrors),
}

impl fmt::Display for UltimateError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            UltimateError::Database(db_error) => write!(fmt, "{db_error}"),
            UltimateError::Auth(auth_error) => write!(fmt, "{auth_error}"),
        }
    }
}

impl Error for UltimateError {}

impl error::ResponseError for UltimateError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UltimateError::Database(db_error) => db_error.error_response(),
            UltimateError::Auth(auth_error) => auth_error.error_response(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UltimateError::Database(db_error) => db_error.status_code(),
            UltimateError::Auth(auth_error) => auth_error.status_code(),
        }
    }
}
