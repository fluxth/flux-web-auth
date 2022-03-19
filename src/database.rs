use rocket::{Build, Rocket};
use rocket_sync_db_pools::database;

#[database("auth_db")]
pub struct AuthDatabase(diesel::PgConnection);

pub async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // This macro from `diesel_migrations` defines an `embedded_migrations`
    // module containing a function named `run` that runs the migrations in the
    // specified directory, initializing the database.
    embed_migrations!("./migrations");

    let conn = AuthDatabase::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");

    rocket
}
