use crate::{
    errors::{self, UltimateError},
    models::user::UserForInvite,
    repository::sql::establish_connection,
};
use actix_web::{
    get,
    web::{self, Json},
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserParams {
    pub user_id: i32,
}

#[get("/user/{user_id}")]
pub async fn func(params: web::Path<UserParams>) -> Result<Json<UserForInvite>, UltimateError> {
    use crate::schema::users::dsl::*;

    let mut connection = match establish_connection() {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::UltimateError::Database(
                errors::DatabaseErrors::CantEstablishConnection(err.to_string()),
            ))
        }
    };

    let userss: Vec<UserForInvite> = match users
        .filter(id.eq(params.user_id))
        .select(UserForInvite::as_select())
        .load(&mut connection)
    {
        Ok(o) => o,
        Err(err) => {
            return Err(UltimateError::Database(
                errors::DatabaseErrors::SelectError(err.to_string()),
            ));
        }
    };

    let user = match userss.into_iter().next() {
        Some(o) => o,
        None => {
            return Err(UltimateError::Database(errors::DatabaseErrors::NoSuchUser(
                "User not found in database (geting single user)".to_string(),
            )));
        }
    };

    Ok(Json(user))
}
