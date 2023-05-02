use std::{error::Error, fmt};

use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

#[derive(Debug)]
pub enum DatabaseErrors {
    SelectError(String),
    CantEstablishConnection(String),
}

impl fmt::Display for DatabaseErrors {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("MySQL database error")
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
        }
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            DatabaseErrors::CantEstablishConnection(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DatabaseErrors::SelectError(_) => StatusCode::BAD_REQUEST,
        }
    }
}
