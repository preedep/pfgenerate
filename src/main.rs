mod authen;
mod proxy;

use actix_web::{App, HttpServer, web};
use log::info;
use crate::proxy::page_handler::page_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    pretty_env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/page_handler")
                .route(web::get().to(page_handler))
                .route(web::post().to(page_handler)))

    })
        .bind(("127.0.0.1", 8888))?
        .run()
        .await

}
