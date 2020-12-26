use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/cats")]
async fn get_cats() -> impl Responder {
    HttpResponse::Ok().body("<div><h1>KRISSY</h1></div>")
}

#[post("/cats")]
async fn post_cats() -> impl Responder {
    HttpResponse::Ok().body("<h2>DUSYA</h2>")
}