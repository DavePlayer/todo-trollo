use crate::{
    errors::{self, UltimateError},
    models::user::{UserClaims, UserForInvite},
    repository::sql::establish_connection,
};
use actix_web::{get, web::Json, HttpMessage, HttpRequest};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

#[get("/users-for-invite")]
pub async fn func(req: HttpRequest) -> Result<Json<Vec<UserForInvite>>, errors::UltimateError> {
    let claims = match req.extensions_mut().get::<UserClaims>() {
        Some(o) => o.clone(),
        None => {
            return Err(UltimateError::Auth(errors::AuthErrors::NoClaimsProvided(
                "User Claims not provided in GET /groups".to_string(),
            )));
        }
    };

    use crate::schema::users::dsl::*;

    let mut connection = match establish_connection() {
        Ok(o) => o,
        Err(err) => {
            return Err(UltimateError::Database(
                errors::DatabaseErrors::CantEstablishConnection(err.to_string()),
            ))
        }
    };

    let invite_users: Vec<UserForInvite> = match users
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

    let invite_users = invite_users
        .into_iter()
        .filter(|x| x.id != claims.id)
        .collect();

    Ok(Json(invite_users))
}
