use super::DatabaseErrors;
use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use std::{error::Error, fmt};

impl fmt::Display for DatabaseErrors {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DatabaseErrors::UserExists(_) => fmt.write_str("User Already Exists"),
            Self::UserNotFound(_) => {
                fmt.write_str(format!("Invalid Credentials. No such user in database").as_str())
            }
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
            DatabaseErrors::NoClaimsProvided(info) => {
                log::error!("claims in request not provided: {}", info);
            }
            DatabaseErrors::UserNotFound(user) => {
                log::error!("User Not found When loging: {}", user.login);
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
            DatabaseErrors::NoClaimsProvided(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DatabaseErrors::UserNotFound(_) => StatusCode::FORBIDDEN,
        }
    }
}
