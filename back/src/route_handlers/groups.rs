use crate::{
    errors,
    models::{
        group::Grup,
        user::{User, UserClaims},
    },
    repository::sql::establish_connection,
    schema::{group_assigned_users, users},
};
use actix_web::{get, web::Json, HttpMessage, HttpRequest};
use diesel::{associations::HasTable, prelude::*};
use jwt::claims;

#[get("/groups")]
pub async fn get_groups(req: HttpRequest) -> Result<Json<Vec<Grup>>, errors::DatabaseErrors> {
    log::info!("fetching groups");
    let claims = match req.extensions_mut().get::<UserClaims>() {
        Some(o) => o.clone(),
        None => {
            return Err(errors::DatabaseErrors::NoClaimsProvided(
                "User Claims not provided in GET /groups".to_string(),
            ));
        }
    };
    log::debug!("User Claims in Request: {:?}", claims);
    use crate::schema::grups::dsl::*;
    let mut connection = match establish_connection() {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::DatabaseErrors::CantEstablishConnection(
                err.to_string(),
            ))
        }
    };

    // no right_join which is bullshit
    // chatGPT remade query with inner joins (tried with left but sth was going wrong)
    // for future this select must be like this. In other cases it screams like banshee
    let grps: Vec<(Grup, User)> = match group_assigned_users::table
        // .left_join(grups.on(id.eq(group_assigned_users::group_id)))
        // .filter(group_assigned_users::user_id.eq(claims.id))
        .inner_join(grups.on(id.eq(group_assigned_users::group_id)))
        .inner_join(users::table.on(users::id.eq(group_assigned_users::user_id)))
        .filter(group_assigned_users::user_id.eq(claims.id))
        .select((grups::all_columns(), users::table::all_columns()))
        .load::<(Grup, User)>(&mut connection)
    {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::DatabaseErrors::SelectError(err.to_string()));
        }
    };

    // remaping vector of (Grup, User) to vector of just Grup
    let grps = grps
        .into_iter()
        .map(|(grp, _)| return grp)
        .collect::<Vec<Grup>>();

    log::info!("found groups: \n{:#?}", grps);

    return Ok(Json(grps));
}
