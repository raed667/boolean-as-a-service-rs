use rand::Rng;
use uuid::Uuid;

mod db;

use db::{
    delete_boolean_by_id, establish_connection, get_boolean_by_id, get_total_count, insert_boolean,
    update_boolean_by_id,
};

use db::models::BooleanModel;

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

pub fn get_boolean(id: String) -> Result<BooleanModel, diesel::result::Error> {
    let connection = &mut establish_connection();

    get_boolean_by_id(connection, id)
}

pub fn delete_boolean(id: String) {
    let connection = &mut establish_connection();
    delete_boolean_by_id(connection, id);
}

pub fn get_count() -> i64 {
    let connection = &mut establish_connection();
    get_total_count(connection)
}
