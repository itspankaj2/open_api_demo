use std::collections::HashMap;

use axum::{
    extract::{self, State},
    response::IntoResponse,
    response::Response,
};
use diesel::expression::is_aggregate::No;
use s3::{Bucket, BucketConfiguration, Region};

use crate::users0::controller;

async fn _login() -> Response {
    todo!()
}

async fn _logout() -> Response {
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

pub async fn upload_to_minio() -> Response {
    use s3::creds::Credentials;

    let access_key = Some("");
    let secret_key = Some("");
    // let security_token = None;
    // let session_token = None;
    // let profile: Option<String> = None;

    let cred = std::env::var("MINIO_CRED");

    if cred.is_err() {
        return (axum::http::StatusCode::UNAUTHORIZED, "No Cred").into_response();
    }

    let cred = cred.unwrap();
    println!("{}", cred);

    let s3cred: HashMap<String, String> = serde_json::from_str(&cred).unwrap();
    let access_key: &str = s3cred.get("accessKey").unwrap();
    let secret_key: &str = s3cred.get("secretKey").unwrap();

    let bucker_name = "hello";
    let s3cred = Credentials::new(Some(access_key), Some(secret_key), None, None, None).unwrap();

    let bucket = Bucket::new(
        bucker_name,
        Region::Custom {
            region: "auto".to_owned(),
            endpoint: "https://minios3.itspankaj.com".to_owned(),
        },
        s3cred.clone(),
    )
    .unwrap();

    let bucket = Bucket::with_path_style(&bucket);

    let (_, code) = bucket.head_object("/").await.unwrap();

    if code == 404 {
        let create_result = Bucket::create_with_path_style(
            bucket.name.as_str(),
            bucket.region.clone(),
            s3cred,
            BucketConfiguration::default(),
        )
        .await
        .unwrap();

        println!(
            "=== Bucket created\n{} - {} - {}",
            bucket.name, create_result.response_code, create_result.response_text
        );
    }

    // 3) Create object (text/plain)
    let key = "test_file_2";
    println!("=== Put content");
    bucket
        .put_object_with_content_type(key, "NEW !!! Stuff!!!".as_bytes(), "text/plain")
        .await
        .unwrap();

    // 4) List bucket content
    println!("=== List bucket content");
    let results = bucket
        .list("/".to_owned(), Some("/".to_owned()))
        .await
        .unwrap();
    for result in results {
        for item in result.contents {
            println!("key: {}", item.key);
        }
    }

    // 5) Get object content back
    println!("=== Get content");
    let x = bucket.get_object(key).await.unwrap();
    let data = x.to_string().unwrap();
    println!("data: {}", data);

    let status = axum::http::StatusCode::CREATED;
    let header: axum::http::HeaderMap = axum::http::HeaderMap::new();

    let body = "".to_owned();
    (status, header, body).into_response()
}
