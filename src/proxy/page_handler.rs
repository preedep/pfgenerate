use actix_web::{HttpResponse, Responder};

pub async fn page_handler() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}