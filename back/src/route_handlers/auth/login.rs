use crate::{
    errors::{self, DatabaseErrors},
    models::user::{User, UserAsResponse, UserClaims, UserToLogin},
    repository::sql::establish_connection,
};
use actix_web::{get, web::Json};
use chrono::Utc;
use diesel::prelude::*;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;

#[get("/login")]
pub async fn login_user(
    body: Json<UserToLogin>,
) -> Result<Json<UserAsResponse>, errors::DatabaseErrors> {
    log::info!("login user {}", body.login);
    log::debug!("{:?}", body);
    use crate::schema::users::dsl::*;

    // establish connection with database
    let mut connection = match establish_connection() {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::DatabaseErrors::CantEstablishConnection(
                err.to_string(),
            ));
        }
    };

    let usr: Vec<User> = match users
        .filter(login.eq(&body.login))
        .filter(password.eq(&body.password))
        .load::<User>(&mut connection)
    {
        Ok(o) => o,
        Err(err) => return Err(errors::DatabaseErrors::SelectError(err.to_string())),
    };
    let usr = match usr.into_iter().nth(0) {
        Some(o) => o,
        None => {
            return Err(DatabaseErrors::UserNotFound(body.0));
        }
    };

    // generating JWT token signed with JWT_SECRET env variable
    let secret = match std::env::var("JWT_SECRET") {
        Ok(s) => s,
        Err(err) => {
            // if env is not set the entire server is not going to work, so it should panic
            panic!("ENV variable error(JWT_SECRET):{:?}", err.to_string());
        }
    };
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
    // let mut claims = BTreeMap::new();
    // claims.insert("sub", "someone");
    let claims = UserClaims {
        id: usr.id.clone(),
        name: usr.name.clone(),
        creation_time: Utc::now(),
    };
    let token_str = claims.sign_with_key(&key).unwrap();

    log::debug!("token: {}", token_str);
    log::info!("successfully authenticated user {} ", &body.login,);
    Ok(Json(UserAsResponse::new(
        usr.id,
        usr.name.as_str(),
        usr.login.as_str(),
        token_str,
    )))
}
