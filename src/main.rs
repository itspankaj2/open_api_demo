use std::{net::SocketAddr, sync::Arc};

use axum::routing::{get, post};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use open_api_demo::pet;

#[tokio::main]
async fn main() {
    let pool: Arc<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>> =
        Arc::new(open_api_demo::connection::get_pool().clone());

    // Create a new user
    let _name = "John Doe";
    let _email = "john@example.com";
    let _password = "123456";

    //users0::api::create_user(&mut connection, name, email, password);

    #[derive(utoipa::OpenApi)]
    #[openapi(
        paths(
            crate::pet::api::get_pet_by_id,
            open_api_demo::users0::api::register,
            // crate::pet::api::get_pet_object
        ),
        components(
            schemas(open_api_demo::pet::api::Pet),
            schemas(open_api_demo::models::User),
            schemas(open_api_demo::models::NewUser),
            // schemas(open_api_demo::schema0::object::Object)
        ),
        tags(
            (name = "todo", description = "Todo items management API")
        )
    )]
    struct ApiDoc;

    let app = axum::Router::new()
        .route("/", get(hello))
        .route("/pets/:id", get(pet::api::get_pet_by_id))
        .route("/api/register", post(open_api_demo::users0::api::register))
        .with_state(pool)
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
