use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Serialize, Queryable, ToSchema)]
pub struct BooleanModel {
    pub id: String,
    pub value: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
