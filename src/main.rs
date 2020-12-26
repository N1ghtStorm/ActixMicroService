use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[path = "./Controllers/cat_controller.rs"]
mod cat_controller;
#[path = "./Controllers/register_controllers.rs"]
mod register_controllers;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            //.register_controllers()
            .service(cat_controller::get_cats)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}