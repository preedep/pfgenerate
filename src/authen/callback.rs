use actix_session::Session;
use actix_web::{Responder, web};
use log::debug;
use tracing_attributes::instrument;

use crate::models::configuration::Config;
use crate::utils::utils::redirect_to_page;

///
///  main page
///
//#[instrument]
#[instrument(skip(session))]
pub async fn callback(
    session: Session,
    data: web::Data<Config>,
) -> impl Responder {
    debug!("Callback called");
    redirect_to_page("")
}
