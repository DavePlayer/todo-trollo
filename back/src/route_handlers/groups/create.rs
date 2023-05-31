use crate::{
    errors,
    models::{
        group::{Grup, NewGroup},
        user::UserClaims,
    },
    repository::sql::establish_connection,
};
use actix_web::{post, web::Json, HttpMessage, HttpRequest};
use diesel::{insert_into, prelude::*};

#[post("/group-add")]
pub async fn create_group(
    req: HttpRequest,
    body: Json<NewGroup>,
) -> Result<String, Box<dyn std::error::Error>> {
    log::info!("inserting new group: {}", body.name);
    let claims = match req.extensions_mut().get::<UserClaims>() {
        Some(o) => o.clone(),
        None => {
            return Err(Box::new(errors::AuthErrors::NoClaimsProvided(
                "User Claims not provided in GET /groups".to_string(),
            )));
        }
    };
    log::debug!("User Claims in Request: {:?}", claims);
    use crate::schema::grups::dsl::*;
    let mut connection = match establish_connection() {
        Ok(o) => o,
        Err(err) => {
            return Err(Box::new(errors::DatabaseErrors::CantEstablishConnection(
                err.to_string(),
            )))
        }
    };

    let grps: Vec<Grup> = match grups
        .filter(name.eq(&body.name))
        .filter(creator.eq(&body.creator))
        .load::<Grup>(&mut connection)
    {
        Ok(o) => o,
        Err(err) => {
            return Err(Box::new(errors::DatabaseErrors::SelectError(
                err.to_string(),
            )));
        }
    };

    if !grps.is_empty() {
        return Err(Box::new(errors::DatabaseErrors::GroupExist(
            grps.into_iter().next().unwrap(),
        )));
    }

    let sth = match insert_into(grups).values(&body.0).execute(&mut connection) {
        Ok(o) => o,
        Err(err) => {
            return Err(Box::new(errors::DatabaseErrors::InsertError(
                err.to_string(),
            )));
        }
    };

    log::info!("created new group(status: {}): \n{:?}", sth, body.0);

    Ok("successfully created new group".to_string())
}
