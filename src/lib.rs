pub mod connection;
pub mod models;
pub mod pet;
pub mod schema;
pub mod schema0;
pub mod users0;

use std::sync::Arc;
pub type POOL = Arc<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>>;
