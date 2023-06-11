use actix_web::web::Json;

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
    UserNotFound(UserToLogin),
    GroupExist(Grup),
    DataNotFound(String),
}
mod database_errors;

#[derive(Debug)]
pub enum AuthErrors {
    InvalidToken(String),
    EnvError(String),
    NoClaimsProvided(String),
}
mod auth_errors;
