use crate::{
    errors::{self},
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
) -> Result<String, errors::UltimateError> {
    log::info!("inserting new group: {}", body.name);
    let claims = match req.extensions_mut().get::<UserClaims>() {
        Some(o) => o.clone(),
        None => {
            return Err(errors::UltimateError::Auth(
                errors::AuthErrors::NoClaimsProvided(
                    "User Claims not provided in GET /groups".to_string(),
                ),
            ));
        }
    };
    log::debug!("User Claims in Request: {:?}", claims);

    use crate::schema::grups::dsl::*;

    let mut connection = match establish_connection() {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::UltimateError::Database(
                errors::DatabaseErrors::CantEstablishConnection(err.to_string()),
            ))
        }
    };

    let grps: Vec<Grup> = match grups
        .filter(name.eq(&body.name))
        .filter(creator.eq(&claims.id))
        .load::<Grup>(&mut connection)
    {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::UltimateError::Database(
                errors::DatabaseErrors::SelectError(err.to_string()),
            ));
        }
    };

    if !grps.is_empty() {
        return Err(errors::UltimateError::Database(
            errors::DatabaseErrors::GroupExist(grps.into_iter().next().unwrap()),
        ));
    }

    let status = match insert_into(grups)
        .values((name.eq(&body.name), creator.eq(&claims.id)))
        .execute(&mut connection)
    {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::UltimateError::Database(
                errors::DatabaseErrors::InsertError(err.to_string()),
            ));
        }
    };

    log::info!("created new group(status: {}): \n{:?}", status, body.0);

    let inserted_grup: Vec<Grup> = match grups
        .filter(name.eq(&body.name))
        .filter(creator.eq(&claims.id))
        .load::<Grup>(&mut connection)
    {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::UltimateError::Database(
                errors::DatabaseErrors::SelectError(err.to_string()),
            ));
        }
    };
    let inserted_grup = match inserted_grup.into_iter().next() {
        Some(o) => o,
        None => {
            return Err(errors::UltimateError::Database(
                errors::DatabaseErrors::DataNotFound(
                    "can't get inserted group back after creating new one".to_string(),
                    "Internal SErver Error. Check logs".to_string(),
                ),
            ));
        }
    };

    use crate::schema::group_assigned_users::dsl::*;

    let status = match insert_into(group_assigned_users)
        .values((group_id.eq(inserted_grup.id), user_id.eq(&claims.id)))
        .execute(&mut connection)
    {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::UltimateError::Database(
                errors::DatabaseErrors::InsertError(err.to_string()),
            ));
        }
    };

    log::info!(
        "inserted creator({}) to newly created group({}): {}",
        &claims.id,
        inserted_grup.name,
        status
    );

    Ok("successfully created new group".to_string())
}
