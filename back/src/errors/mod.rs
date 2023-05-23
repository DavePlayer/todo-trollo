use std::{error::Error, fmt};

use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    web::Json,
    HttpResponse,
};

use crate::models::user::UserToRegister;

#[derive(Debug)]
pub enum DatabaseErrors {
    SelectError(String),
    CantEstablishConnection(String),
    UserExists(Json<UserToRegister>),
    InsertError(String),
}

impl fmt::Display for DatabaseErrors {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DatabaseErrors::UserExists(_) => fmt.write_str("User Already Exists"),
            _ => fmt.write_str("MySQL database error"),
        }
    }
}

impl Error for DatabaseErrors {}

impl error::ResponseError for DatabaseErrors {
    fn error_response(&self) -> HttpResponse {
        match self {
            DatabaseErrors::CantEstablishConnection(err) => {
                log::error!("couldn't establish connection with database:\n {}", err)
            }
            DatabaseErrors::SelectError(err) => {
                log::error!("error when using select statement to database:\n {}", err)
            }
            DatabaseErrors::InsertError(err) => {
                log::error!("error when using insert statement to database:\n {}", err)
            }
            DatabaseErrors::UserExists(user) => {
                log::error!(
                    "Tried to register existing user: login: {} | name: {}",
                    user.login,
                    user.name
                )
            }
        }
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            DatabaseErrors::CantEstablishConnection(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DatabaseErrors::SelectError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DatabaseErrors::InsertError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DatabaseErrors::UserExists(_) => StatusCode::CONFLICT,
        }
    }
}
