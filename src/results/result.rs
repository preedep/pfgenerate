use std::fmt::{Debug, Display, Formatter};

use actix_web::{error, HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WebResponse {}

#[derive(Debug, Serialize)]
pub struct WebResponseError {}

impl Responder for WebResponse {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

impl Display for WebResponseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl error::ResponseError for WebResponseError {
    fn status_code(&self) -> StatusCode {
        /*
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }*/
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

pub type WebResult<T> = Result<T, WebResponseError>;
