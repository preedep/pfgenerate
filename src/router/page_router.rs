use actix_web::{HttpRequest, web};
use actix_web::http::Method;
use futures_util::future::err;
use log::{debug, error};
use serde::Deserialize;

use crate::results::result::{WebResponse, WebResponseError, WebResult};

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

pub async fn page_handler(req: HttpRequest) -> WebResult<WebResponse> {
    debug!("Calling page handler");
    return match req.method() {
        &Method::GET => {
            debug!("Call GET page handler");
            do_get_page_handler(req.clone()).await
        },
        &Method::POST => {
            debug!("Call POST page handler");
            do_post_page_handler(req.clone()).await
        },
        _ => {
            Ok(WebResponse {})
        }
    }
    /*
    return match req.method(){
        None => {
            Err(WebResponseError {})
        }
        Some(s) => {
            match s {
                "GET" => {
                    debug!("Call GET page handler");
                    do_get_page_handler(req.clone()).await
                }
                "POST" => {
                    debug!("Call POST page handler");
                    do_post_page_handler(req.clone()).await
                }
                _ => {
                    Ok(WebResponse {})
                }
            }
        }
    };

     */
}

async fn do_get_page_handler(req: HttpRequest) -> WebResult<WebResponse> {
    let params = web::Query::<PageHandlerParams>::from_query(
        req.query_string()
    )
        .unwrap_or(web::Query(PageHandlerParams {
            page_from: None,
            page_to: None,
        }));

    do_internal_page_handler(params.0).await
}

async fn do_post_page_handler(req: HttpRequest) -> WebResult<WebResponse> {
    let params = web::Query::<PageHandlerParams>::from_query(
        req.query_string()
    )
        .unwrap_or(web::Query(PageHandlerParams {
            page_from: None,
            page_to: None,
        }));

    do_internal_page_handler(params.0).await
}

async fn do_internal_page_handler(page_params: PageHandlerParams) -> WebResult<WebResponse> {
    Ok(WebResponse {})
}