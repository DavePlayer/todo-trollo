#[macro_use]
extern crate diesel;

use actix_web::{get, middleware::Logger, web::Data, App, HttpServer};

mod errors;
mod models;
mod repository;
mod route_handlers;
mod schema;

#[get("/")]
async fn index() -> String {
    format!("sup bro!") // <- response with app_name
}

#[actix_web::main()]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::debug!("works");
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            // .app_data(db_data)
            .service(index)
            .service(route_handlers::groups::get_groups)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
