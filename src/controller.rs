use actix_web::{
    delete, get, post, put,
    web::{Json, Path, ServiceConfig},
    HttpResponse, Responder,
};

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    models::{CreateBoolean, UpdateBoolean},
    service::{
        create_boolean, create_boolean_random, delete_boolean, get_boolean, get_count,
        update_boolean,
    },
};

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub(super) enum ErrorResponse {
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
}

#[derive(Serialize, ToSchema)]
struct CountResponse {
    count: i64,
}

pub(super) fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(create) //
            .service(random) //
            .service(delete) //
            .service(update) //
            .service(get_one) //
            .service(count_booleans); //
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get a count")
    )
)]
#[get("/count")]
async fn count_booleans() -> impl Responder {
    let count = get_count();

    let response = CountResponse { count };

    HttpResponse::Ok().json(response)
}

#[utoipa::path(
    params(
        ("id", description = "Unique storage id of Boolean")
    ),
    responses(
        (status = 200, description = "Get a boolean", body = [BooleanModel])
    )
)]
#[get("/boolean/{id}")]
async fn get_one(path: Path<String>) -> impl Responder {
    let id = path.into_inner();

    let result = get_boolean(id);

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
    let boolean = create_boolean(body.value);
    HttpResponse::Created().json(boolean)
}

#[utoipa::path(
    responses(
        (status = 201, description = "Create a random boolean", body=BooleanModel)
    )
)]
#[post("/random")]
async fn random() -> impl Responder {
    let boolean = create_boolean_random();
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
    delete_boolean(id);
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

    update_boolean(id, body.value)
        .map(|boolean| HttpResponse::Ok().json(boolean))
        .unwrap_or_else(|_| {
            HttpResponse::NotFound().json(ErrorResponse::NotFound(format!("id not found")))
        })
}
