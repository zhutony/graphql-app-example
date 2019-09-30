pub mod pagination;

use crate::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use juniper_eager_loading::impl_load_from_for_diesel;

#[derive(Queryable, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub country_id: i32,
}

#[derive(Queryable, Debug, Clone)]
pub struct Country {
    pub id: i32,
    pub name: String,
}

impl_load_from_for_diesel! {
    (
        error = diesel::result::Error,
        connection = PgConnection,
    ) => {
        i32 -> (users, User),
        i32 -> (countries, Country),
    }
}
