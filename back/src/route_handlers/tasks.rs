use std::error::Error;

use crate::{
    errors,
    models::{taks::Task, user::UserClaims},
    repository::sql::establish_connection,
};
use actix_web::{
    get,
    web::{self, Json},
    HttpMessage, HttpRequest,
};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TaskParams {
    pub group_id: i32,
}

#[get("/tasks/{group_id}")]
pub async fn get_tasks_by_group_id(
    req: HttpRequest,
    task_params: web::Path<TaskParams>,
) -> Result<Json<Vec<Task>>, Box<dyn Error>> {
    log::info!("fetching tasks from group_id: {}", task_params.group_id);
    let claims = match req.extensions_mut().get::<UserClaims>() {
        Some(o) => o.clone(),
        None => {
            return Err(Box::new(errors::AuthErrors::NoClaimsProvided(
                "User Claims not provided in GET /groups".to_string(),
            )));
        }
    };
    log::debug!("User Claims in Request: {:?}", claims);

    use crate::schema::tasks::dsl::*;
    let mut connection = match establish_connection() {
        Ok(o) => o,
        Err(err) => {
            return Err(Box::new(errors::DatabaseErrors::CantEstablishConnection(
                err.to_string(),
            )))
        }
    };

    // no right_join which is bullshit
    // chatGPT remade query with inner joins (tried with left but sth was going wrong)
    // for future this select must be like this. In other cases it screams like banshee
    let tsk = match tasks
        .filter(group_id.eq(task_params.group_id))
        .load::<Task>(&mut connection)
    {
        Ok(o) => o,
        Err(err) => {
            return Err(Box::new(errors::DatabaseErrors::SelectError(
                err.to_string(),
            )));
        }
    };

    // remaping vector of (Grup, User) to vector of just Grup

    Ok(Json(tsk))
}
