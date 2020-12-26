use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder };

#[path = "./cat_controller.rs"]
mod cat_controller;


// pub fn register_controllers<T, B>(app: &App<T, B>) -> &App<T, B> {
//     *app
//     .service(cat_controller::get_cats)
//     .service(cat_controller::post_cats)
// }

// pub fn register_controllers() -> App<AppEntry, Body> {
//     App::new()
//     //.register_controllers()
//     .service(cat_controller::get_cats)
//     .service(cat_controller::post_cats)
// }


