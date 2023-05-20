use std::net::SocketAddr;

use axum::routing::get;
use diesel::{Connection, PgConnection};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod models;
mod pet;
mod schema;
mod schema0;
mod users0;

#[tokio::main]
async fn main() {
    let _connection = PgConnection::establish("my_database.db").unwrap();

    // Create a new user
    let _name = "John Doe";
    let _email = "john@example.com";
    let _password = "123456";

    //users0::api::create_user(&mut connection, name, email, password);

    #[derive(utoipa::OpenApi)]
    #[openapi(
        paths(
            pet::api::get_pet_by_id,
            pet::api::get_pet_object
        ),
        components(
            schemas(pet::api::Pet),
            schemas(schema0::object::Object)
        ),
        tags(
            (name = "todo", description = "Todo items management API")
        )
    )]
    struct ApiDoc;

    let app = axum::Router::new()
        .route("/", get(hello))
        .route("/pets/:id", get(pet::api::get_pet_by_id))
        .route("/schema/:name", get(pet::api::get_pet_object))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let address = SocketAddr::new([127, 0, 0, 1].into(), 3333);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn hello() -> String {
    return "Hello".to_string();
}
