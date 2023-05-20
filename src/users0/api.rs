use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::{
    models::{NewUser, User},
    schema,
};

pub fn create_user<'a>(
    _conn: &mut PgConnection,
    name1: &'a str,
    email1: &'a str,
    password1: &'a str,
) -> Result<User, diesel::result::Error> {
    let new_user = NewUser {
        name: name1,
        email: email1,
        password: password1,
    };

    use schema::users;
    let x = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(_conn);
    return x;
}

fn read_users(conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    use schema::users::dsl::*;

    users.limit(10).load::<User>(conn)
}

fn update_user(
    conn: &mut PgConnection,
    user_id: i32,
    new_name: &str,
) -> Result<usize, diesel::result::Error> {
    use schema::users::dsl::*;

    diesel::update(users.find(user_id))
        .set(name.eq(new_name))
        .execute(conn)
}

fn delete_user(conn: &mut PgConnection, user_id: i32) -> Result<usize, diesel::result::Error> {
    use schema::users::dsl::*;

    diesel::delete(users.find(user_id)).execute(conn)
}
