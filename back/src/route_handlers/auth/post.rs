use crate::{
    errors::{self, DatabaseErrors},
    models::user::{User, UserAsResponse, UserClaims, UserToRegister},
    repository::sql::establish_connection,
    websockets::server::ChatServer,
};
use actix::Addr;
use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest,
};
use chrono::Utc;
use diesel::{insert_into, prelude::*};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;

#[post("/register")]
pub async fn register_new_user(
    data: Data<Addr<ChatServer>>,
    _req: HttpRequest,
    body: Json<UserToRegister>,
) -> Result<Json<UserAsResponse>, errors::UltimateError> {
    log::info!("registering new user {} | {}", body.login, body.name);
    use crate::schema::users::dsl::*;

    // establish connection with database
    let mut connection = match establish_connection() {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::UltimateError::Database(
                errors::DatabaseErrors::CantEstablishConnection(err.to_string()),
            ));
        }
    };

    // check if user is already in database
    let checked_users = match users
        .filter(login.eq(&body.login))
        .filter(name.eq(&body.name))
        .load::<User>(&mut connection)
    {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::UltimateError::Database(
                errors::DatabaseErrors::SelectError(err.to_string()),
            ))
        }
    };

    log::debug!("users: {:?}", checked_users);

    // return 409 if there is any user matching with login or name
    if !checked_users.is_empty() {
        return Err(errors::UltimateError::Database(
            errors::DatabaseErrors::UserExists(body),
        ));
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
            return Err(errors::UltimateError::Database(
                DatabaseErrors::InsertError(err.to_string()),
            ));
        }
    };

    // MySQL is the only database which does not support last id in diesel
    // so i have to select user again so i can get only id from it
    // unplesant but works
    // somehow
    let usr: Vec<User> = match users
        .filter(login.eq(&body.login))
        .filter(name.eq(&body.name))
        .load::<User>(&mut connection)
    {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::UltimateError::Database(
                errors::DatabaseErrors::SelectError(err.to_string()),
            ))
        }
    };
    let usr = match usr.into_iter().next() {
        Some(o) => o,
        None => {
            return Err(errors::UltimateError::Database(
                DatabaseErrors::SelectError("can't get single user from db".to_string()),
            ));
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
        id: usr.id,
        name: body.name.as_str().to_string(),
        creation_time: Utc::now(),
    };
    let token_str = claims.sign_with_key(&key).unwrap();

    log::debug!("token: {}", token_str);
    log::info!(
        "successfully registered new user {} | {} -> status: {}",
        &body.login,
        &body.name,
        ans
    );

    let data = data.get_ref();
    data.do_send(crate::websockets::server::ClientMessage {
        id: 1,
        msg: format!("/registered {}", &body.name),
        room: "Main".to_string(),
    });

    Ok(Json(UserAsResponse::new(
        usr.id,
        body.name.as_str(),
        body.login.as_str(),
        token_str,
    )))
}
