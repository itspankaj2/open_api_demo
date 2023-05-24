use dotenv::dotenv;

pub fn get_connection() -> diesel::PgConnection {
    use diesel::{Connection, PgConnection};
    dotenv().ok();
    let env_database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _connection = PgConnection::establish(env_database_url.as_str()).unwrap();
    return _connection;
}

pub fn get_test_connection() -> diesel::PgConnection {
    use diesel::{Connection, PgConnection};
    dotenv().ok();
    let env_database_url = std::env::var("DATABASE_TEST_URL").expect("DATABASE_URL must be set");
    let _connection = PgConnection::establish(env_database_url.as_str()).unwrap();
    return _connection;
}

pub fn get_pool() -> diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>> {
    use diesel::r2d2::ConnectionManager;

    dotenv().ok();
    let env_database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<diesel::pg::PgConnection>::new(env_database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    return pool;
}
