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
            Self::UserNotFound(_) => fmt.write_str("Invalid Credentials. No such user in database"),
            Self::GroupExist(_) => fmt.write_str("group already exists"),
            Self::DataNotFound(_) => fmt.write_str("There is no data of such type"),
            Self::DataExists(_) => fmt.write_str("Data already exists"),
            Self::AlreadyInGroup(_) => {
                fmt.write_str("one of the given user is already in one of the group")
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
                log::error!("couldn't establish connection with database:\n {}", err);
            }
            DatabaseErrors::SelectError(err) => {
                log::error!("error when using select statement to database:\n {}", err);
            }
            DatabaseErrors::InsertError(err) => {
                log::error!("error when using insert statement to database:\n {}", err);
            }
            DatabaseErrors::UserExists(user) => {
                log::error!(
                    "Tried to register existing user: login: {} | name: {}",
                    user.login,
                    user.name
                );
            }
            DatabaseErrors::UserNotFound(user) => {
                log::error!("User Not found When loging: {}", user.login);
            }
            DatabaseErrors::GroupExist(grp) => {
                log::error!("Group already exist: {}", grp.name);
            }
            DatabaseErrors::DataNotFound(info) => {
                log::error!("Couldn't get data: {}", info);
            }
            DatabaseErrors::DataExists(info) => {
                log::error!("trying to insert existing data: {}", info);
            }
            DatabaseErrors::AlreadyInGroup(info) => {
                log::error!(
                    "Tried to assign one of the users to group that he is already in: {:?}",
                    info
                );
            }
        };
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
            DatabaseErrors::UserNotFound(_) => StatusCode::FORBIDDEN,
            DatabaseErrors::GroupExist(_) => StatusCode::CONFLICT,
            DatabaseErrors::DataNotFound(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DatabaseErrors::DataExists(_) => StatusCode::CONFLICT,
            DatabaseErrors::AlreadyInGroup(_) => StatusCode::CONFLICT,
        }
    }
}
