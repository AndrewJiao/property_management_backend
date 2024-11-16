use actix_web::web;
use diesel::PgConnection;

pub mod error;
pub mod conf;
pub mod const_value;
pub mod web_config;
pub mod db_config;


pub fn build_app_context(context_name: &str) -> AppContextBuilder {
    AppContextBuilder {
        name: Some(context_name.to_string()),
        db_connection: None,
    }
}

pub struct AppContext {
    name: String,
    db_connection: PgConnection,
}

pub  struct AppContextBuilder {
    name: Option<String>,
    db_connection: Option<PgConnection>,
}
impl AppContextBuilder {
    pub fn db_connection(mut self, connection: PgConnection) -> Self {
        self.db_connection = Some(connection);
        self
    }

    pub fn build_app_data(self) -> web::Data<AppContext> {
        web::Data::new(
            AppContext {
                name: self.name.unwrap(),
                db_connection: self.db_connection.expect("db_connection is required"),
            }
        )
    }
}


