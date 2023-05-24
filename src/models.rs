use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::schema::users;

#[derive(Debug, ToSchema, diesel::Queryable, Serialize, Deserialize)]
pub struct User {
    pub userid: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub dob: Option<i32>,
    pub location: Option<String>,
}

#[derive(diesel::Insertable, ToSchema, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}
