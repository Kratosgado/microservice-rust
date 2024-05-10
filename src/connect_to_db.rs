use std::env;

use diesel::{Connection, PgConnection};

const DEFAULT_DATABASE_URL: &'static str = "postgresql://postgres@localhost:5432";

pub fn connect_to_db() -> Option<PgConnection> {
    let database_url = env::var("DATABASE_URL").unwrap_or(String::from(DEFAULT_DATABASE_URL));
    match PgConnection::establish(&database_url) {
        Ok(conn) => Some(conn),
        Err(err) => {
            error!("Error connecting to database: {}", err);
            None
        }
    }
}