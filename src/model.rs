use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub enum Payment {
    PENDING,
    DONE
}

#[derive(Debug, Insertable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = crate::schema::orders)]
pub struct NewOrder {

    pub delivery_address: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::orders)]
pub struct Order {
    pub id : i32,
    pub product_id: i32,
    pub payment: String,
    pub delivery_address: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>, 
}

