pub mod error;
pub mod conf;
pub mod const_value;
pub mod web_config;
pub mod data_result;
pub mod tools;
pub mod process_macro_derive;
pub mod db_config;

pub const CURRENT_USE: &str = "System";

// pub fn build_app_context() -> AppContextBuilder {
//     AppContextBuilder {
//         db_connection: None,
//     }
// }
//
// pub struct AppContext {
//     db_connection: PgConnection,
// }
//
// pub struct AppContextBuilder {
//     db_connection: Option<PgConnection>,
// }
// impl AppContextBuilder {
//     pub fn db_connection(mut self, connection: PgConnection) -> Self {
//         self.db_connection = Some(connection);
//         self
//     }
//
//     pub fn build_app_data(self) -> web::Data<AppContext> {
//         web::Data::new(
//             AppContext {
//                 db_connection: self.db_connection.expect("db_connection is required"),
//             }
//         )
//     }
// }


