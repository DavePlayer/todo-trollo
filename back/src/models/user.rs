use crate::schema::users;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserToRegister {
    pub name: String,
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserToLogin {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAsResponse {
    pub id: i32,
    pub name: String,
    pub login: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserClaims {
    pub id: i32,
    pub name: String,
    pub creation_time: chrono::DateTime<Utc>,
}

impl UserAsResponse {
    pub fn new(id: i32, name: &str, login: &str, token: String) -> UserAsResponse {
        UserAsResponse {
            id,
            name: name.to_string(),
            login: login.to_string(),
            token,
        }
    }
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

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = users)]
pub struct UserForInvite {
    pub id: i32,
    pub name: String,
    pub img_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGroupInvite {
    pub user_ids: Vec<i32>,
    pub group_id: i32,
}
