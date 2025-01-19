use crate::db_config::{db_get_connection, Conn};
use diesel::result::Error;
use diesel::Connection;

pub trait TryTransaction {
    fn try_transaction<T, E, F>(self, f: F) -> Result<T, E>
    where
            for<'a> F: FnOnce(&'a mut Conn) -> Result<T, E>,
            E: From<Error>;
}

impl TryTransaction for Option<&mut Conn> {
    fn try_transaction<T, E, F>(self, f: F) -> Result<T, E>
    where
            for<'a> F: FnOnce(&'a mut Conn) -> Result<T, E>,
            E: From<Error>,
    {
        match self {
            Some(conn) => {
                conn.transaction(|conn| f(conn))
            }
            None => {
                let conn: &mut Conn = &mut db_get_connection();
                conn.transaction(|conn| f(conn))
            }
        }
    }
}