use actix_web::{
    delete, get, post, put,
    web::{Json, Path, ServiceConfig},
    HttpResponse, Responder,
};
use db::establish_connection;
use db::get_boolean_by_id;

use serde::Deserialize;
use serde::Serialize;

use utoipa::ToSchema;

#[path = "./db.rs"]
mod db;

#[path = "./service.rs"]
mod service;

use service::CreateBoolean;
use service::UpdateBoolean;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub(super) enum ErrorResponse {
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
}

pub(super) fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(create) //
            .service(random) //
            .service(delete) //
            .service(update) //
            .service(get_one); //
    }
}

#[utoipa::path(
    params(
        ("id", description = "Unique storage id of Boolean")
    ),
    responses(
        (status = 200, description = "Get a boolean", body = [BooleanModel])
    )
)]
#[get("/{id}")]
async fn get_one(path: Path<String>) -> impl Responder {
    let id = path.into_inner();
    let connection = &mut establish_connection();

    let result = get_boolean_by_id(connection, id);

    result
        .map(|boolean| HttpResponse::Ok().json(boolean))
        .unwrap_or_else(|_| {
            HttpResponse::NotFound().json(ErrorResponse::NotFound(format!("id not found")))
        })
}

#[utoipa::path(
    request_body = CreateBoolean,
    responses(
        (status = 201, description = "Create a boolean", body=BooleanModel)
    )
)]
#[post("/create")]
async fn create(body: Json<CreateBoolean>) -> impl Responder {
    let boolean = service::create_boolean(body.value);
    HttpResponse::Created().json(boolean)
}

#[utoipa::path(
    responses(
        (status = 201, description = "Create a random boolean", body=BooleanModel)
    )
)]
#[post("/random")]
async fn random() -> impl Responder {
    let boolean = service::create_boolean_random();
    HttpResponse::Created().json(boolean)
}

#[utoipa::path(
    params(
        ("id", description = "Unique storage id of Boolean")
    ),
    responses(
        (status = 204, description = "Delete a boolean")
    )
)]
#[delete("/{id}")]
async fn delete(path: Path<(String,)>) -> impl Responder {
    let id = path.into_inner().0;
    let connection = &mut establish_connection();
    db::delete_boolean_by_id(connection, id);

    HttpResponse::NoContent()
}

#[utoipa::path(
    request_body = UpdateBoolean,
     params(
        ("id", description = "Unique storage id of Boolean")
    ),
    responses(
        (status = 200, description = "Update a boolean", body=BooleanModel)
    )
)]
#[put("/{id}")]
async fn update(path: Path<String>, body: Json<UpdateBoolean>) -> impl Responder {
    let id = path.into_inner();

    println!("path {}", id);

    service::update_boolean(id, body.value)
        .map(|boolean| HttpResponse::Ok().json(boolean))
        .unwrap_or_else(|_| {
            HttpResponse::NotFound().json(ErrorResponse::NotFound(format!("id not found")))
        })
}
