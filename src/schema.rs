// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct OrdersPaymentEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::OrdersPaymentEnum;

    orders (id) {
        id -> Integer,
        order_number -> Integer,
        PersonID -> Nullable<Integer>,
        product_id -> Nullable<Integer>,
        #[max_length = 255]
        delivery_address -> Varchar,
        #[max_length = 7]
        payment -> OrdersPaymentEnum,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    products (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        price -> Integer,
        #[max_length = 255]
        descri -> Varchar,
        #[max_length = 100]
        part_number -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        shopkeeper_id -> Integer,
    }
}

diesel::table! {
    shopkeepers (id) {
        id -> Integer,
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        username -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        #[max_length = 10]
        phone_number -> Nullable<Char>,
        #[max_length = 255]
        shop_name -> Nullable<Varchar>,
        #[max_length = 255]
        shop_address -> Nullable<Varchar>,
        #[max_length = 255]
        city -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        username -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        #[max_length = 10]
        phone_number -> Char,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(orders, products, shopkeepers, users,);
