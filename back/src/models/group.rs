use crate::schema::grups;
use serde::Serialize;

#[derive(Serialize, Insertable)]
#[diesel(table_name = grups)]
pub struct NewGroup {
    pub id: i32,
    pub name: String,
    pub creator: i32,
}

#[derive(Debug, Queryable, AsChangeset, Serialize)]
pub struct Grup {
    pub id: i32,
    pub name: String,
    pub creator: i32,
}
