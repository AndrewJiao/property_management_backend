use diesel::Connection;
use diesel::result::Error;
use crate::db_config::{db_get_connection, Conn};

fn try_transaction<T, E, F>(f: F) -> Result<T, E>
where
    F: FnOnce() -> Result<T, E>,
    E: From<Error>,
{
    let conn = &mut db_get_connection();
}


pub trait TryTransaction {
    fn try_transaction<T, E, F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
        E: From<Error>;
}

impl TryTransaction for Option<&mut Conn> {
    fn try_transaction<T, E, F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
        E: From<Error>,
    {
        match self {
            Some(conn) => {
                conn.transaction(f)
            }
            None => {
                let conn = &mut db_get_connection();
                conn.transaction(f)
            }
        }
    }
}