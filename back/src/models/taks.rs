use crate::schema::tasks;
use serde::Serialize;

#[derive(Serialize, Insertable, Queryable)]
#[diesel(table_name = tasks)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub crossed_by_id: Option<i32>,
    pub group_id: i32,
}
