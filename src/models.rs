use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::result::Error;

use crate::schema;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn find_username(conn: &PgConnection, username: &str) -> Result<Self, Error> {
        use schema::users::dsl;
        dsl::users
            .filter(dsl::username.eq(username))
            .first::<Self>(conn)
    }
}
