use crate::{
    errors::{self, UltimateError},
    models::{
        taks::{Task, TaskToCross},
        user::UserClaims,
    },
    repository::sql::establish_connection,
    websockets::server::ChatServer,
};
use actix::Addr;
use actix_web::{
    patch,
    web::{Data, Json},
    HttpMessage, HttpRequest,
};
use diesel::{dsl::sql, prelude::*};

#[patch("/cross")]
pub async fn cross_task(
    data: Data<Addr<ChatServer>>,
    req: HttpRequest,
    body: Json<TaskToCross>,
) -> Result<String, errors::UltimateError> {
    let data = data.get_ref();
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
    log::info!("crossing task {} by user id: {}", body.id, claims.id);
    log::debug!("User Claims in Request: {:?}", claims);

    // establish connection with database
    let mut connection = match establish_connection() {
        Ok(o) => o,
        Err(err) => {
            return Err(UltimateError::Database(
                errors::DatabaseErrors::CantEstablishConnection(err.to_string()),
            ))
        }
    };

    use crate::schema::tasks::dsl::*;

    let task_in_server: Vec<Task> =
        match tasks.filter(id.eq(&body.id)).load::<Task>(&mut connection) {
            Ok(o) => o,
            Err(err) => {
                return Err(errors::UltimateError::Database(
                    errors::DatabaseErrors::SelectError(err.to_string()),
                ));
            }
        };

    if task_in_server.is_empty() {
        return Err(errors::UltimateError::Database(
            errors::DatabaseErrors::DataNotFound(
                format!("No such task with given id in cross-task: {}", body.id),
                "Trying to cross task that does not exist".to_string(),
            ),
        ));
    }

    let task_in_server = task_in_server.into_iter().next().unwrap();
    // to many variables called task_id
    if let Some(penis) = task_in_server.crossed_by_id {
        let update_status = match diesel::update(tasks)
            .filter(id.eq(body.id))
            .set(crossed_by_id.eq(sql("NULL")))
            .execute(&mut connection)
        {
            Ok(o) => o,
            Err(err) => {
                return Err(errors::UltimateError::Database(
                    errors::DatabaseErrors::UpdateError(err.to_string()),
                ));
            }
        };
        let task_in_server: Vec<Task> =
            match tasks.filter(id.eq(&body.id)).load::<Task>(&mut connection) {
                Ok(o) => o,
                Err(err) => {
                    return Err(errors::UltimateError::Database(
                        errors::DatabaseErrors::SelectError(err.to_string()),
                    ));
                }
            };
        let task_in_server = task_in_server.into_iter().next().unwrap();

        log::info!("successfully de-crossed task({}): {}", penis, update_status);

        data.do_send(crate::websockets::server::ClientMessage {
            id: 1,
            msg: format!(
                "/taskUncrossed {}",
                match serde_json::to_string(&task_in_server) {
                    Ok(o) => o,
                    Err(_) => "{}".to_string(),
                }
            ),
            room: "Main".to_string(),
        });

        return Ok("Successfully de-crossed task".to_string());
    }

    let update_status = match diesel::update(tasks)
        .filter(id.eq(body.id))
        .set(crossed_by_id.eq(claims.id))
        .execute(&mut connection)
    {
        Ok(o) => o,
        Err(err) => {
            return Err(errors::UltimateError::Database(
                errors::DatabaseErrors::UpdateError(err.to_string()),
            ));
        }
    };

    log::info!("successfully crossed task: {}", update_status);

    let task_in_server: Vec<Task> =
        match tasks.filter(id.eq(&body.id)).load::<Task>(&mut connection) {
            Ok(o) => o,
            Err(err) => {
                return Err(errors::UltimateError::Database(
                    errors::DatabaseErrors::SelectError(err.to_string()),
                ));
            }
        };
    let task_in_server = task_in_server.into_iter().next().unwrap();

    data.do_send(crate::websockets::server::ClientMessage {
        id: 1,
        msg: format!(
            "/taskCrossed {}",
            match serde_json::to_string(&task_in_server) {
                Ok(o) => o,
                Err(_) => "{}".to_string(),
            }
        ),
        room: "Main".to_string(),
    });

    Ok("Successfully crossed task".to_string())
}
