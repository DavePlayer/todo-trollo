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
