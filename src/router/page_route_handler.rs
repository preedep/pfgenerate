use actix_web::{HttpRequest, web};
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
    page_from: PageType,
    #[serde(flatten)]
    page_to: PageType,
}

pub async fn page_handler(req: HttpRequest) -> WebResult<WebResponse> {
    return match req.match_name() {
        None => {
            Err(WebResponseError {})
        }
        Some(s) => {
            match s {
                "GET" => {
                    do_get_page_handler(req.clone()).await
                }
                "POST" => {
                    do_post_page_handler(req.clone()).await
                }
                _ => {
                    Ok(WebResponse {})
                }
            }
        }
    };
}

async fn do_get_page_handler(req: HttpRequest) -> WebResult<WebResponse> {
    let params = web::Query::<PageHandlerParams>::from_query(
        req.query_string()
    )
        .unwrap_or(web::Query(PageHandlerParams {
            page_from: PageType::LOGIN,
            page_to: PageType::LOGIN,
        }));

    do_internal_page_handler(params.0).await
}

async fn do_post_page_handler(req: HttpRequest) -> WebResult<WebResponse> {
    let params = web::Query::<PageHandlerParams>::from_query(
        req.query_string()
    )
        .unwrap_or(web::Query(PageHandlerParams {
            page_from: PageType::LOGIN,
            page_to: PageType::LOGIN,
        }));

    do_internal_page_handler(params.0).await
}

async fn do_internal_page_handler(page_params: PageHandlerParams) -> WebResult<WebResponse> {
    Ok(WebResponse {})
}