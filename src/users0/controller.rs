use axum::response::{IntoResponse, Response};

use crate::models::User;

pub fn success_registration(user: User) -> Response {
    let status = axum::http::StatusCode::CREATED;
    let header: axum::http::HeaderMap = axum::http::HeaderMap::new();

    let body = serde_json::to_string(&user).unwrap_or("Failed to desealize".to_owned());
    (status, header, body).into_response()
}

pub fn db_known_error_registration(error: diesel::result::DatabaseErrorKind) -> Response {
    let status = axum::http::StatusCode::CREATED;
    let header: axum::http::HeaderMap = axum::http::HeaderMap::new();

    match error {
        diesel::result::DatabaseErrorKind::CheckViolation => todo!(),
        diesel::result::DatabaseErrorKind::ForeignKeyViolation => todo!(),
        diesel::result::DatabaseErrorKind::UniqueViolation => {
            let body = "User already present".to_owned();
            return (axum::http::StatusCode::NOT_MODIFIED, header, body).into_response();
        }
        _ => todo!(),
    };

    let body = "".to_owned();
    (status, header, body).into_response()
}

pub fn db_unknown_error_registration() -> Response {
    let status = axum::http::StatusCode::CREATED;
    let header: axum::http::HeaderMap = axum::http::HeaderMap::new();

    let body = "".to_owned();
    (status, header, body).into_response()
}
