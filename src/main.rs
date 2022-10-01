extern crate diesel;
extern crate env_logger;

use std::{io::Result, net::Ipv4Addr};

use actix_web::{middleware::Logger, App, HttpServer};

#[path = "./models.rs"]
mod models;

#[path = "./service.rs"]
mod service;

use models::{BooleanModel, CreateBoolean, UpdateBoolean};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

mod controller;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    #[derive(OpenApi)]
    #[openapi(
        paths(
            controller::get_one,
            controller::create,
            controller::random,
            controller::delete,
            controller::update,
            controller::count_booleans,
        ),
        components(
           schemas(BooleanModel, CreateBoolean, UpdateBoolean)
        ),
        tags(
            (name = "Boolean-as-a-Service", description = "Boolean management endpoints.")
        ),
    )]
    struct ApiDoc;

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(controller::configure())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()),
            )
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}
