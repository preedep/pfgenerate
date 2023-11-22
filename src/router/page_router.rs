use actix_web::{FromRequest, HttpRequest, Responder, web};
use actix_web::http::Method;
use log::debug;
use serde::Deserialize;

use crate::utils::utils::redirect_to_page;

#[derive(Debug, Deserialize)]
pub enum PageType {
    LOGIN,
    LOGOUT,
}

#[derive(Debug, Deserialize)]
pub struct PageHandlerParams {
    #[serde(flatten)]
    page_from: Option<PageType>,
    #[serde(flatten)]
    page_to: Option<PageType>,
}

pub async fn page_handler(req: HttpRequest) -> impl Responder {
    debug!("Calling page handler");
    let params = match req.method() {
        &Method::GET => {
            debug!("Call GET page handler");
            web::Query::<PageHandlerParams>::from_query(
                req.query_string()
            ).unwrap().0
        }
        &Method::POST => {
            debug!("Call POST page handler");
            web::Form::<PageHandlerParams>::extract(&req).await.unwrap().0
        }
        _ => {
            debug!("Call GET page handler");
            PageHandlerParams {
                page_from: None,
                page_to: None,
            }
        }
    };
    redirect_to_page("/")
}