use crate::{
    errors::{self, DatabaseErrors},
    models::user::{User, UserToRegister},
    repository::sql::establish_connection,
};
use actix_web::{post, web::Json};
use diesel::{insert_into, prelude::*};

#[post("/register")]
pub async fn register_new_user(
    body: Json<UserToRegister>,
) -> Result<Json<Vec<UserToRegister>>, errors::DatabaseErrors> {
    log::info!("registering new user {} | {}", body.login, body.name);
    log::debug!("{:?}", body);
    use crate::schema::users::dsl::*;
    let mut connection = match establish_connection() {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::DatabaseErrors::CantEstablishConnection(
                err.to_string(),
            ))
        }
    };

    let checked_users = match users
        .filter(login.eq(&body.login))
        .filter(name.eq(&body.name))
        .load::<User>(&mut connection)
    {
        Ok(o) => o,
        Err(err) => return Err(errors::DatabaseErrors::SelectError(err.to_string())),
    };

    log::debug!("users: {:?}", checked_users);

    if checked_users.len() > 0 {
        return Err(errors::DatabaseErrors::UserExists(body));
    }

    let ans = match insert_into(users)
        .values((
            name.eq(&body.name),
            login.eq(&body.login),
            password.eq(&body.password),
        ))
        .execute(&mut connection)
    {
        Ok(sth) => sth,
        Err(err) => {
            return Err(DatabaseErrors::InsertError(err.to_string()));
        }
    };

    log::info!(
        "successfully registered new user {} | {} -> status: {}",
        body.login,
        body.name,
        ans
    );

    Ok(Json(Vec::new()))
}