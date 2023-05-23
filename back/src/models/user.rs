use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserToRegister {
    pub name: String,
    pub login: String,
    pub password: String,
}

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub login: String,
    pub password: String,
    pub img_url: Option<String>,
}
