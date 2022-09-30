extern crate diesel;

use actix_web::{App, HttpServer};

#[path = "./models.rs"]
mod models;

use models::BooleanModel;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            controller::get_one,
            controller::create,
            controller::random,
            controller::delete,
            controller::update,
        ),
        components(
           schemas(BooleanModel)
        ),
        tags(
            (name = "Boolean-as-a-Service", description = "Boolean management endpoints.")
        ),
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new().configure(controller::configure()).service(
            SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
