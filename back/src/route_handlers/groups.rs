use crate::{
    errors,
    models::{group::Grup, user::UserClaims},
    repository::sql::establish_connection,
};
use actix_web::{get, web::Json, HttpMessage, HttpRequest};
use diesel::prelude::*;

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

    let grps = match grups.load::<Grup>(&mut connection) {
        Ok(o) => o,
        Err(err) => return Err(errors::DatabaseErrors::SelectError(err.to_string())),
    };

    log::info!("found groups");

    return Ok(Json(grps));
}
