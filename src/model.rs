use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Debug, Insertable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = crate::schema::orders)]
pub struct NewOrder {
    pub delivery_address: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::orders)]
pub struct Order{
 pub delivery_address: String,
  pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}