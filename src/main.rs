use actix_session::{SessionExt, SessionMiddleware};
use actix_session::config::PersistentSession;
use actix_session::storage::RedisActorSessionStore;
use actix_web::{App, cookie, HttpServer, middleware, web};
use actix_web::cookie::SameSite;
use actix_web::cookie::time::Duration;
use actix_web::dev::Service as _;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use futures_util::future::FutureExt;
use log::{debug, info};

use crate::authen::callback::callback;
use crate::authen::login::login;
use crate::models::configuration::Config;
use crate::models::entra_id::{JWKS, OpenIDConfigurationV2};
use crate::pages::error::page_error;
use crate::pages::index::page_index;
use crate::router::page_router::page_handler;

mod authen;
mod router;
mod models;
mod results;
mod pages;
mod utils;

const APP_AUTHEN_SESSION_KEY: &'static str = "APP_AUTHEN_SESSION_KEY";

fn middle_ware_session(
    redis_connection: &str,
    private_key: cookie::Key,
    use_cookie_ssl: bool,
) -> SessionMiddleware<RedisActorSessionStore> {
    let redis_connection = redis_connection.replace("redis://", "");
    debug!("Redis uri: {}", redis_connection);

    SessionMiddleware::builder(RedisActorSessionStore::new(
        redis_connection),
                               private_key)
        .cookie_name(APP_AUTHEN_SESSION_KEY.to_string())
        .session_lifecycle(
            PersistentSession::default().session_ttl(Duration::minutes(15 /*1 day*/)),
        )
        .cookie_secure(use_cookie_ssl)
        .cookie_same_site(SameSite::None)
        .cookie_http_only(true)
        .build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    let redis_url = std::env::var("REDIS_URL").unwrap();
    let redis_auth_key = std::env::var("REDIS_AUTH_KEY").unwrap();
    let tenant_id = std::env::var("TENANT_ID").unwrap();
    let default_page = std::env::var("DEFAULT_PAGE").unwrap();
    let redirect_url = std::env::var("REDIRECT_URL").unwrap();
    let client_id = std::env::var("CLIENT_ID").unwrap();
    let client_secret = std::env::var("CLIENT_SECRET").unwrap();
    let cookie_ssl = std::env::var("COOKIE_SSL").unwrap_or("false".to_string());
    let use_cookie_ssl: bool = cookie_ssl.parse::<bool>().unwrap_or(false);


    let mut config = Config::new(
        redis_url.clone(),
        redis_auth_key.clone(),
        tenant_id.clone(),
        default_page.clone(),
        redirect_url.clone(),
        client_id.clone(),
        client_secret.clone(),
    );

    debug!("Config loaded successfully = {:#?}", config);

    //
    // Get azure ad meta data
    //
    let url_openid_config = format!(
        r#"https://login.microsoftonline.com/{:1}/v2.0/.well-known/openid-configuration?appid={:2}"#,
        config.to_owned().tenant_id,
        config.to_owned().client_id
    );

    info!("url get azure ad configuration : {}", url_openid_config);

    let res_meta_data_entra_id = reqwest::get(url_openid_config)
        .await
        .unwrap()
        .json::<OpenIDConfigurationV2>()
        .await;

    match res_meta_data_entra_id {
        Ok(entra_id_info) => {
            config.open_id_config = Some(entra_id_info);
            debug!("Entra ID = {:#?}",config.open_id_config);
            debug!("Get JWKS configuration");
            //get JWKS for verify jwt token
            let jwks_uri = config.open_id_config.clone().unwrap().jwks_uri.unwrap();
            let res_jwks_items = reqwest::get(jwks_uri).await.unwrap().json::<JWKS>().await;
            match res_jwks_items {
                Ok(jwks) => {
                    debug!("Entra JWKS = {:#?}",jwks);
                    config.jwks = Some(jwks)
                }
                Err(er) => {
                    panic!("{}", er)
                }
            }

            // generate private key for session
            let private_key = actix_web::cookie::Key::generate();
            HttpServer::new(move || {
                App::new()
                    .app_data(Data::new(config.clone()))
                    .wrap(middleware::DefaultHeaders::new().add(("Dev-X-Version", "0.1")))
                    .wrap(Logger::default())
                    .wrap(Logger::new(
                        r#"%a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#,
                    ))
                    .wrap(middle_ware_session(
                        config.clone().redis_url.as_str(),
                        private_key.clone(),
                        use_cookie_ssl,
                    ))
                    .wrap_fn(|req, srv| {
                        debug!("Path request : {}",req.path());
                        let mut is_logon = false;
                        if !req.path().eq("/authentication") && !req.path().eq("/callback") {
                            debug!("Is not /authentication and /callback");
                            let cookie = req.cookie(APP_AUTHEN_SESSION_KEY);
                            match cookie {
                                None => {
                                    debug!("Cookie not found");
                                }
                                Some(cookie) => {
                                    let expire = cookie.expires();
                                    match expire {
                                        None => {
                                            debug!("Expiration expired not found");
                                        }
                                        Some(expire) => {
                                            debug!("Expiration expired {:?}", expire);
                                            is_logon = true;
                                        }
                                    }
                                }
                            }
                        }
                        let fut = srv.call(req);
                        async {
                            let mut res = fut.await?;
                            /*
                            Ok(res.into_response(
                                redirect_to_page("/authentication").map_into_boxed_body(),
                            ))*/
                            Ok(res)
                        }
                        ////
                    })
                    .route("/",web::get().to(page_index))
                    .route("/authentication", web::get().to(login))
                    .route("/error",web::get().to(page_error))
                    .route("/callback",web::post().to(callback))
                    .service(web::resource("/pagerouting")
                        .route(web::get().to(page_handler))
                        .route(web::post().to(page_handler)))
            }).workers(10)
                .bind(("0.0.0.0", 8888))?
                .run()
                .await
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}
