use axum::{
    extract::{self, State},
    response::Response,
};

use crate::{models::User, users0::controller};

async fn login() -> Response {
    todo!()
}

async fn logout() -> Response {
    todo!()
}

#[utoipa::path(
    post,
    path = "/api/register",
    request_body(content = open_api_demo::models::NewUser, description = "User to register", content_type = "application/json"),
    responses(
        (status = 200, description = "User Registered successfully", body = open_api_demo::models::User, content_type = "application/json" ),
        (status = 401, description = "Body format is not correct"),
        (status = NOT_MODIFIED, description = "Email Already exsit")
    )
)]
pub async fn register(
    State(pool): State<crate::POOL>,
    extract::Json(payload): extract::Json<crate::models::NewUser>,
) -> Response {
    let mut conn = pool.get().unwrap();

    let result =
        super::model::create_user(&mut conn, payload.name, payload.email, payload.password);

    match result {
        Ok(user) => controller::success_registration(user),
        Err(diesel::result::Error::DatabaseError(error_kind, _)) => {
            controller::db_known_error_registration(error_kind)
        }

        Err(_) => controller::db_unknown_error_registration(),
    }
}
