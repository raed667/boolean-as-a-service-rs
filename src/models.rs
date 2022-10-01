use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;
use utoipa::ToSchema;

#[derive(Clone, Serialize, Queryable, ToSchema, IntoParams)]
pub struct BooleanModel {
    pub id: String,
    pub value: bool,
    #[schema(value_type = String)]
    // #[param(allow_reserved, value_type = String, example = "2022-09-21T22:02:33")]
    pub created_at: NaiveDateTime,
    #[schema(value_type = String)]
    // #[param(allow_reserved, value_type = String, example = "2022-09-21T22:02:33")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateBoolean {
    #[schema(value_type = bool)]
    pub value: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateBoolean {
    pub value: bool,
}
