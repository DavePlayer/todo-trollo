use actix_web::web::Json;

use crate::models::user::{UserToLogin, UserToRegister};

#[derive(Debug)]
pub enum DatabaseErrors {
    SelectError(String),
    CantEstablishConnection(String),
    UserExists(Json<UserToRegister>),
    InsertError(String),
    NoClaimsProvided(String),
    UserNotFound(UserToLogin),
}
mod database_errors;

#[derive(Debug)]
pub enum AuthErrors {
    InvalidToken(String),
    EnvError(String),
}
mod auth_errors;
