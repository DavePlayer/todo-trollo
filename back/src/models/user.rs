use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserToRegister {
    pub name: String,
    pub login: String,
    pub password: String,
}

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub login: String,
    pub password: String,
    pub img_url: String,
}
