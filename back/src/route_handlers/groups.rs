use crate::{errors, models::group::Grup, repository::sql::establish_connection};
use actix_web::{get, web::Json};
use diesel::prelude::*;

#[get("/groups")]
pub async fn get_groups() -> Result<Json<Vec<Grup>>, errors::DatabaseErrors> {
    log::info!("fetching groups");
    use crate::schema::grups::dsl::*;
    let mut connection = match establish_connection() {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::DatabaseErrors::CantEstablishConnection(
                err.to_string(),
            ))
        }
    };
    log::debug!("established conenction");

    let grps = match grups.load::<Grup>(&mut connection) {
        Ok(o) => o,
        Err(err) => return Err(errors::DatabaseErrors::SelectError(err.to_string())),
    };

    return Ok(Json(grps));
}
