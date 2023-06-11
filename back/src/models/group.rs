use crate::schema::group_assigned_users;
use crate::schema::grups;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Insertable, Deserialize, Debug)]
#[diesel(table_name = grups)]
pub struct NewGroup {
    pub name: String,
    pub creator: i32,
}

#[derive(Debug, Queryable, AsChangeset, Serialize)]
pub struct Grup {
    pub id: i32,
    pub name: String,
    pub creator: i32,
}

#[derive(Serialize, Insertable, Deserialize, Debug, Queryable)]
#[diesel(table_name = group_assigned_users)]
pub struct GroupAssignedUsers {
    pub id: i32,
    pub group_id: i32,
    pub user_id: i32,
}
