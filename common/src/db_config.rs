use crate::const_value::SETTINGS;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = &SETTINGS.databases.database_url;
    let manage = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manage)
        .expect("Failed to create pool.")
}

pub fn establish_connection_str(database_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manage = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manage)
        .expect("Failed to create pool.")
}

