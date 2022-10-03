use diesel::dsl::now;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::BooleanModel;
use crate::schema::booleans;
use crate::schema::booleans::dsl::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Insertable)]
#[diesel(table_name = booleans)]
pub struct NewBoolean {
    pub id: String,
    pub value: bool,
}

pub fn insert_boolean(
    conn: &mut SqliteConnection,
    id_new: String,
    value_new: bool,
) -> BooleanModel {
    let new_boolean = NewBoolean {
        id: id_new,
        value: value_new,
    };

    let boolean = diesel::insert_into(booleans::table)
        .values(&new_boolean)
        .get_result::<BooleanModel>(conn)
        .expect("Error insert_boolean");

    return boolean;
}

pub fn get_boolean_by_id(
    conn: &mut SqliteConnection,
    idq: String,
) -> Result<BooleanModel, diesel::result::Error> {
    let result = booleans.filter(id.eq(idq)).first::<BooleanModel>(conn);

    return result;
}

pub fn delete_boolean_by_id(conn: &mut SqliteConnection, idq: String) {
    diesel::delete(booleans.filter(id.eq(idq)))
        .execute(conn)
        .unwrap();
}

pub fn update_boolean_by_id(
    conn: &mut SqliteConnection,
    idq: String,
    valueq: bool,
) -> Result<BooleanModel, diesel::result::Error> {
    return diesel::update(booleans)
        .filter(id.eq(idq))
        .set((value.eq(valueq), updated_at.eq(now)))
        .get_result::<BooleanModel>(conn);
}

pub fn get_total_count(conn: &mut SqliteConnection) -> i64 {
    let cute_cat_count: i64 = booleans
        .filter(id.is_not("AAAA"))
        .count()
        .get_result(conn)
        .expect("Error counting cute kittens");

    println!("XXXXXX {}", cute_cat_count);

    return cute_cat_count;
}
