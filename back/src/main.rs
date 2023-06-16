#[macro_use]
extern crate diesel;

use std::sync::{atomic::AtomicUsize, Arc};

use actix::Actor;

#[allow(unused_imports)]
use actix_files as fs;

use actix_web::{
    get,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;

extern crate dotenv;
use dotenv::dotenv;

use crate::websockets::chat_route;

mod errors;
mod middlewares;
mod models;
mod repository;
mod route_handlers;
mod schema;
mod tests;
mod websockets;

#[get("/")]
async fn index() -> String {
    "sup bro!".to_string() // <- response with app_name
}

#[actix_web::main()]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    log::debug!("works");
    let app_state = Arc::new(AtomicUsize::new(0));
    let server = websockets::server::ChatServer::new(app_state.clone()).start();
    HttpServer::new(move || {
        let logger = Logger::default();
        // App state
        // We are keeping a count of the number of visitors
        let bearre_middleware = HttpAuthentication::bearer(middlewares::validate_jwt::validator);
        // Start chat server actor
        App::new()
            .wrap(logger)
            .app_data(Data::new(app_state.clone()))
            .app_data(Data::new(server.clone()))
            // .app_data(db_data)
            .service(index)
            // websocket
            .service(web::resource("/ws/").to(chat_route))
            // static resources
            // .service(fs::Files::new("/static/", "static/"))
            .service(
                web::scope("/auth")
                    .service(route_handlers::auth::post::register_new_user)
                    .service(route_handlers::auth::get::login_user),
            )
            .service(
                web::scope("")
                    // .app_data(
                    //     bearer::Config::default()
                    //         .realm("Restricted area")
                    //         .scope("email photo"),
                    // )
                    .service(route_handlers::groups::get::get_groups)
                    .service(route_handlers::groups::post::create_group)
                    .service(route_handlers::tasks::get::get_tasks_by_group_id)
                    .service(route_handlers::tasks::post::create_task)
                    .service(web::scope("/task").service(route_handlers::tasks::patch::cross_task))
                    .service(route_handlers::users_handle::get::get_users_for_invite::func)
                    .service(route_handlers::users_handle::get::get_single_user::func)
                    .service(route_handlers::users_handle::update::force_assign_users)
                    .wrap(bearre_middleware),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
