use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use tracing_attributes::instrument;

use crate::models::configuration::Config;
use crate::models::web_params::ErrorParameter;


#[instrument(skip(session))]
pub async fn page_error(
    session: Session,
    params: web::Query<ErrorParameter>,
    data: web::Data<Config>,
) -> impl Responder {
    HttpResponse::Ok().finish()
}