use actix_session::Session;
use actix_web::web;
use tracing_attributes::instrument;
use crate::models::configuration::Config;
use crate::results::result::{WebResponse, WebResult};

///
///  main page
///
//#[instrument]
#[instrument(skip(_session))]
pub async fn callback(
    _session: Session,
    _data: web::Data<Config>,
) -> WebResult<WebResponse> {
    Ok(WebResponse {})
}
