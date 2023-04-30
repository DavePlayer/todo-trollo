use actix_web::{get, App, HttpServer};

#[get("/")]
async fn index() -> String {
    format!("sup bro!") // <- response with app_name
}

#[actix_web::main()]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
