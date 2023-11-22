use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use log::debug;
use tracing_attributes::instrument;

use crate::models::configuration::Config;
use crate::models::entra_id::IDToken;
use crate::utils::utils::KEY_ID_TOKEN;

#[instrument(skip(session))]
pub async fn page_index(
    session: Session,
    data: web::Data<Config>,
) -> impl Responder {
    match session.get::<IDToken>(KEY_ID_TOKEN).unwrap() {
        None => {}
        Some(id_token) => {
            debug!("Page Index with ID token {:#?}", id_token);
        }
    }
    HttpResponse::Ok().finish()
}