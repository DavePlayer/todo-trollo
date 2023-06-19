use crate::{
    errors::{self},
    models::taks::{Task, TaskToInsert},
    repository::sql::establish_connection,
    websockets::server::ChatServer,
};
use actix::Addr;
use actix_web::{
    post,
    web::{Data, Json},
};
use diesel::{insert_into, prelude::*};

#[post("/task-add")]
pub async fn create_task(
    data: Data<Addr<ChatServer>>,
    body: Json<TaskToInsert>,
) -> Result<Json<Task>, errors::UltimateError> {
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

    let tasks_to_return: Vec<Task> = match tasks
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
    let task_to_return = match tasks_to_return.into_iter().next() {
        Some(o) => o,
        None => {
            return Err(errors::UltimateError::Database(
                errors::DatabaseErrors::DataNotFound(
                    "cant get task to emit in to websockets from db".to_string(),
                    "error when returning task".to_string(),
                ),
            ));
        }
    };

    log::info!("created new task({}): {}", &body.name, status);

    let data = data.get_ref();
    match data.try_send(crate::websockets::server::ClientMessage {
        id: 1,
        msg: format!(
            "/taskCreate {}",
            match serde_json::to_string(&task_to_return) {
                Ok(o) => o,
                Err(_) => "{}".to_string(),
            }
        ),
        room: "Main".to_string(),
    }) {
        Ok(_) => {}
        Err(err) => {
            log::error!("couldn't emit message to websockets {}", err.to_string());
        }
    };

    Ok(Json(task_to_return))
}
