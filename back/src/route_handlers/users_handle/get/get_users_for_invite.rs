use crate::{
    errors::{self, UltimateError},
    models::user::{UserClaims, UserForInvite},
    repository::sql::establish_connection,
    schema::group_assigned_users,
    schema::users as usersss,
};
use actix_web::{
    get,
    web::{self, Json},
    HttpMessage, HttpRequest,
};
use diesel::ExpressionMethods;
use diesel::{dsl::not, prelude::*, QueryDsl, SelectableHelper};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    pub group_id: i32,
}

#[get("/users-for-invite/{group_id}")]
pub async fn func(
    params: web::Path<Params>,
    req: HttpRequest,
) -> Result<Json<Vec<UserForInvite>>, errors::UltimateError> {
    let claims = match req.extensions_mut().get::<UserClaims>() {
        Some(o) => o.clone(),
        None => {
            return Err(UltimateError::Auth(errors::AuthErrors::NoClaimsProvided(
                "User Claims not provided in GET /groups".to_string(),
            )));
        }
    };

    let mut connection = match establish_connection() {
        Ok(o) => o,
        Err(err) => {
            return Err(UltimateError::Database(
                errors::DatabaseErrors::CantEstablishConnection(err.to_string()),
            ))
        }
    };

    let not_assigned_users = group_assigned_users::table
        .select(group_assigned_users::user_id)
        .filter(group_assigned_users::group_id.eq(params.group_id));

    let not_assigned_user_ids: Vec<i32> =
        not_assigned_users.load(&mut connection).map_err(|err| {
            UltimateError::Database(errors::DatabaseErrors::SelectError(err.to_string()))
        })?;

    let invite_users: Vec<UserForInvite> = match usersss::table
        .select(UserForInvite::as_select())
        .filter(not(usersss::id.eq_any(&not_assigned_user_ids)))
        .load(&mut connection)
    {
        Ok(o) => o,
        Err(err) => {
            return Err(UltimateError::Database(
                errors::DatabaseErrors::SelectError(err.to_string()),
            ));
        }
    };

    log::debug!(
        "INVITE USERS TO GROUP {}: {:?}",
        params.group_id,
        invite_users
    );

    let invite_users = invite_users
        .into_iter()
        .filter(|x| x.id != claims.id)
        .collect();

    Ok(Json(invite_users))
}
