mod authen;
mod router;
mod models;
mod results;

use actix_web::{App, cookie, HttpServer, middleware, web};
use actix_session::{SessionMiddleware};
use actix_session::config::PersistentSession;
use actix_session::storage::RedisActorSessionStore;
use actix_web::cookie::SameSite;
use actix_web::cookie::time::Duration;
use actix_web::http::Method;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use log::debug;
use crate::models::configuration::Config;
use crate::router::page_route_handler::page_handler;

fn middle_ware_session(
    redis_connection: &str,
    private_key: cookie::Key,
    use_cookie_ssl: bool,
) -> SessionMiddleware<RedisActorSessionStore> {
    SessionMiddleware::builder(RedisActorSessionStore::new(
        redis_connection),
                               private_key)
        .cookie_name("APP_AUTHEN_SESSION_KEY".to_string())
        .session_lifecycle(
            PersistentSession::default().session_ttl(Duration::minutes(15 /*1 day*/)),
        )
        .cookie_secure(use_cookie_ssl)
        .cookie_same_site(SameSite::None)
        .cookie_http_only(true)
        .build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    pretty_env_logger::init();

    let redis_url = std::env::var("REDIS_URL").unwrap();
    let redis_auth_key = std::env::var("REDIS_AUTH_KEY").unwrap();
    let tenant_id = std::env::var("TENANT_ID").unwrap();
    let default_page = std::env::var("DEFAULT_PAGE").unwrap();
    let redirect_url = std::env::var("REDIRECT_URL").unwrap();
    let client_id = std::env::var("CLIENT_ID").unwrap();
    let client_secret = std::env::var("CLIENT_SECRET").unwrap();
    let cookie_ssl = std::env::var("COOKIE_SSL").unwrap_or("false".to_string());

    let config = Config::new(
        redirect_url.clone(),
        redis_auth_key.clone(),
        tenant_id.clone(),
        default_page.clone(),
        redirect_url.clone(),
        client_id.clone(),
        client_secret.clone(),
    );

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.clone()))
            .wrap(middleware::DefaultHeaders::new().add(("Dev-X-Version", "0.1")))
            .wrap(Logger::default())
            .wrap(Logger::new(
                r#"%a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#,
            ))
            .service(web::resource("/pagerouting")
                .route(web::get().to(page_handler))
                .route(web::post().to(page_handler)))

    })
        .bind(("0.0.0.0", 8888))?
        .run()
        .await

}
