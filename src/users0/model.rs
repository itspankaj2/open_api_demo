use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::{
    models::{NewUser, User},
    schema,
};

pub fn create_user(
    conn: &mut PgConnection,
    name1: String,
    email1: String,
    password1: String,
) -> Result<User, diesel::result::Error> {
    let new_user = NewUser {
        name: name1,
        email: email1,
        password: password1,
    };

    use schema::users;
    let x = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn);
    return x;
}

fn read_users(conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    use schema::users::dsl::*;

    users.load::<User>(conn)
}

fn read_user_by_id(conn: &mut PgConnection, user_id: i32) -> Result<User, diesel::result::Error> {
    use schema::users::dsl::*;
    users.find(user_id).first::<User>(conn)
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

fn delete_all_users(conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(schema::users::table).execute(conn)
}

mod tests {

    #[test]
    fn insert_user() {
        let mut conn = crate::connection::get_connection();
        let _ = super::delete_all_users(&mut crate::connection::get_connection());
        let user = super::create_user(
            &mut conn,
            "test".to_owned(),
            "XXXXXXXXXXXXX".to_owned(),
            "test".to_owned(),
        );

        assert!(user.is_ok());

        if let Ok(_user) = user {
            assert_eq!(_user.name, "test");
            assert_eq!(_user.email, "XXXXXXXXXXXXX");
            assert_eq!(_user.password, "test");
        } else {
            assert!(false);
        }

        let _users = super::read_users(&mut conn);
        assert!(_users.is_ok());
        if let Ok(_users) = _users {
            assert_eq!(_users.len(), 1);
        }
    }
}
