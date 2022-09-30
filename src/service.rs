use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::db::models::BooleanModel;

use super::db::{establish_connection, insert_boolean, update_boolean_by_id};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBoolean {
    pub value: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBoolean {
    pub value: bool,
}

pub fn create_boolean_random() -> BooleanModel {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..=2);
    let random_value = number % 2 == 0;

    create_boolean(random_value)
}

pub fn create_boolean(value: bool) -> BooleanModel {
    let id = Uuid::new_v4().to_string();

    let connection = &mut establish_connection();
    let idx = id.clone();

    let model = insert_boolean(connection, idx, value);

    return model;
}

pub fn update_boolean(id: String, value: bool) -> Result<BooleanModel, diesel::result::Error> {
    let connection = &mut establish_connection();
    let idx = id.clone();

    let model = update_boolean_by_id(connection, idx, value);

    return model;
}
