#[macro_use]
extern crate diesel;

use actix_web::{
    get,
    middleware::Logger,
    web::{self},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;

extern crate dotenv;
use dotenv::dotenv;

mod errors;
mod middlewares;
mod models;
mod repository;
mod route_handlers;
mod schema;
mod tests;

#[get("/")]
async fn index() -> String {
    "sup bro!".to_string() // <- response with app_name
}

#[actix_web::main()]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    log::debug!("works");
    HttpServer::new(move || {
        let logger = Logger::default();
        let bearre_middleware = HttpAuthentication::bearer(middlewares::validate_jwt::validator);
        App::new()
            .wrap(logger)
            // .app_data(db_data)
            .service(index)
            .service(
                web::scope("/auth")
                    .service(route_handlers::auth::register::register_new_user)
                    .service(route_handlers::auth::login::login_user),
            )
            .service(
                web::scope("")
                    // .app_data(
                    //     bearer::Config::default()
                    //         .realm("Restricted area")
                    //         .scope("email photo"),
                    // )
                    .service(route_handlers::groups::get::get_groups)
                    .service(route_handlers::groups::create::create_group)
                    .service(route_handlers::tasks::get_tasks_by_group_id)
                    .wrap(bearre_middleware),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
