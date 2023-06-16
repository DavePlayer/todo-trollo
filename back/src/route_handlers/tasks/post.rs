use crate::{
    errors::{self},
    models::taks::{Task, TaskToInsert},
    repository::sql::establish_connection,
};
use actix_web::{post, web::Json};
use diesel::{insert_into, prelude::*};

#[post("/task-add")]
pub async fn create_task(body: Json<TaskToInsert>) -> Result<String, errors::UltimateError> {
    log::info!("creating new task {} in group {}", body.name, body.group_id);
    log::debug!("{:?}", body);

    // establish connection with database
    let mut connection = match establish_connection() {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::UltimateError::Database(
                errors::DatabaseErrors::CantEstablishConnection(err.to_string()),
            ))
        }
    };

    use crate::schema::tasks::dsl::*;

    let tasks_check_vec: Vec<Task> = match tasks
        .filter(name.like(&body.name))
        .filter(group_id.eq(&body.group_id))
        .load::<Task>(&mut connection)
    {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::UltimateError::Database(
                errors::DatabaseErrors::SelectError(err.to_string()),
            ));
        }
    };

    if !tasks_check_vec.is_empty() {
        return Err(errors::UltimateError::Database(
            errors::DatabaseErrors::DataExists(
                "creating task that already exist in some group".to_string(),
                "Task already exist in specified group".to_string(),
            ),
        ));
    }

    let status = match insert_into(tasks).values(&body.0).execute(&mut connection) {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::UltimateError::Database(
                errors::DatabaseErrors::InsertError(err.to_string()),
            ));
        }
    };

    log::info!("created new task({}): {}", &body.name, status);

    Ok("Successfully created new task".to_string())
}
