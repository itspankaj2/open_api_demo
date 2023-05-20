use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schema0;
#[derive(ToSchema, Serialize, Deserialize)]
pub struct Pet {
    pub id: u64,
    pub name: String,
    pub age: Option<i32>,
}

#[utoipa::path(
    get,
    path = "/pets/{id}",
    responses(
        (status = 200, description = "Pet found successfully", body = Pet),
        (status = NOT_FOUND, description = "Pet was not found")
    ),
    params(
        ("id" = u64, Path, description = "Pet database id to get Pet for"),
    )
)]
pub async fn get_pet_by_id(Path(id): Path<u64>) -> Response {
    if id == 4 {
        axum::Json(Pet {
            id: id,
            age: None,
            name: "lightning".to_string(),
        })
        .into_response()
    } else {
        axum::http::status::StatusCode::NOT_FOUND.into_response()
    }
}

#[utoipa::path(
    get,
    path = "/schema/{name}",
    responses(
        (status = 200, description = "Pet found successfully", body = schema::object::Object),
        (status = NOT_FOUND, description = "Pet was not found")
    ),
    params(
        ("name" = String, Path, description = "Pet database id to get Pet for"),
    )
)]
pub async fn get_pet_object(Path(name): Path<String>) -> Response {
    Json(schema0::object::Object::new(&name)).into_response()
}
