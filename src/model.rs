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
    pub product_id: i32,
    pub delivery_address: String,
    pub payment: String
}

#[derive(Debug,Queryable)]
#[diesel(table_name = crate::schema::order_items)]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub user_id: i32,
    pub quantity: i32,
    pub unit_price: i32,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::order_items)]
pub struct NewOrderItems{
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub unit_price: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::orders)]
pub struct Order {
    pub id : i32,
    pub user_id: i32,
    pub payment: String,
    pub delivery_address: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>, 
}

