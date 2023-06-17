use crate::{
    errors::{self, UltimateError},
    models::{
        group::{GroupAssignedUsers, Grup},
        user::UserGroupInvite,
    },
    repository::sql::establish_connection,
    websockets::server::ChatServer,
};
use actix::Addr;
use actix_web::{
    patch,
    web::{Data, Json},
};
use diesel::{insert_into, prelude::*};

#[patch("/force-invite-users")]
pub async fn force_assign_users(
    data: Data<Addr<ChatServer>>,
    body: Json<UserGroupInvite>,
) -> Result<String, errors::UltimateError> {
    log::info!("assigning users {} to group {}", 1, 1);

    // establish connection with database
    let mut connection = match establish_connection() {
        Ok(o) => o,
        Err(err) => {
            return Err(UltimateError::Database(
                errors::DatabaseErrors::CantEstablishConnection(err.to_string()),
            ))
        }
    };

    let group_user_connections: Vec<GroupAssignedUsers> = match group_assigned_users
        .filter(user_id.eq_any(&body.user_ids))
        .filter(group_id.eq(&body.group_id))
        .load::<GroupAssignedUsers>(&mut connection)
    {
        Ok(o) => o,
        Err(err) => {
            return Err(UltimateError::Database(
                errors::DatabaseErrors::SelectError(err.to_string()),
            ))
        }
    };

    if !group_user_connections.is_empty() {
        let user_ids: Vec<i32> = group_user_connections
            .into_iter()
            .map(|x| x.user_id)
            .collect();
        log::error!("penis");
        return Err(UltimateError::Database(
            errors::DatabaseErrors::AlreadyInGroup(user_ids),
        ));
    }

    use crate::schema::group_assigned_users::dsl::*;

    for user_id_t in &body.0.user_ids {
        let status = match insert_into(group_assigned_users)
            .values((group_id.eq(&body.group_id), user_id.eq(user_id_t)))
            .execute(&mut connection)
        {
            Ok(o) => o,
            Err(err) => {
                return Err(UltimateError::Database(
                    errors::DatabaseErrors::InsertError(err.to_string()),
                ));
            }
        };
        log::info!(
            "added user({}) to group({}): {}",
            user_id_t,
            &body.group_id,
            status
        );

        use crate::schema::grups::dsl::*;

        let grps: Vec<Grup> = match grups
            .filter(id.eq(&body.group_id))
            .load::<Grup>(&mut connection)
        {
            Ok(o) => o,
            Err(err) => {
                return Err(errors::UltimateError::Database(
                    errors::DatabaseErrors::SelectError(err.to_string()),
                ));
            }
        };

        let grp = match grps.into_iter().next() {
            Some(o) => o,
            None => {
                return Err(UltimateError::Database(
                    errors::DatabaseErrors::DataNotFound(
                        "can't get group to emit websocket".to_string(),
                        "server error idk".to_string(),
                    ),
                ));
            }
        };

        let data = data.get_ref();
        match data.try_send(crate::websockets::server::ClientMessage {
            id: 1,
            msg: format!(
                "/forceInvite {{user_id: {}, group: {} }}",
                user_id_t,
                match serde_json::to_string(&grp) {
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
    }

    Ok("Successfully invited users".to_string())
}
