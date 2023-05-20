use super::schema::users;

#[derive(Debug, diesel::Queryable)]
pub struct User {
    pub userid: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub dob: Option<i32>,
    pub location: Option<String>,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}
