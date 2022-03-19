use rocket_sync_db_pools::database;

#[database("auth_db")]
pub struct AuthDatabase(diesel::PgConnection);
