use crate::{
    errors::{self, UltimateError},
    models::{group::GroupAssignedUsers, user::UserGroupInvite},
    repository::sql::establish_connection,
};
use actix_web::{patch, web::Json};
use diesel::{insert_into, prelude::*};

#[patch("/force-invite-users")]
pub async fn force_assign_users(
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
    }

    Ok("Successfully invited users".to_string())
}
