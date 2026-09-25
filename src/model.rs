use diesel::{Selectable, deserialize::Queryable, prelude::Insertable};

use crate::schema;

#[derive(Queryable,Selectable,Insertable)]
#[crate::schema::table::orders]
pub struct NewQrder {
    pub st
}