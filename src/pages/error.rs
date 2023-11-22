use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use tracing_attributes::instrument;

use crate::models::configuration::Config;


#[instrument(skip(session))]
pub async fn page_error(
    session: Session,
    data: web::Data<Config>,
) -> impl Responder {
    HttpResponse::Ok().finish()
}