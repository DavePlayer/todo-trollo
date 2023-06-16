use crate::schema::tasks;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Insertable, Queryable, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = tasks)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub crossed_by_id: Option<i32>,
    pub group_id: i32,
}

#[derive(Serialize, Insertable, Queryable, Debug, Deserialize)]
#[diesel(table_name = tasks)]
pub struct TaskToInsert {
    pub name: String,
    pub group_id: i32,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct TaskToCross {
    pub id: i32,
}
