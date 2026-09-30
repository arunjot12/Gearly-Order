use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub delivery_address: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>, 
}

//  id -> Integer,
//         order_number -> Integer,
//         PersonID -> Nullable<Integer>,
//         product_id -> Nullable<Integer>,
//         #[max_length = 255]
//         delivery_address -> Varchar,
//         #[max_length = 7]
//         payment -> OrdersPaymentEnum,
//         created_at -> Nullable<Timestamp>,
//         updated_at -> Nullable<Timestamp>,