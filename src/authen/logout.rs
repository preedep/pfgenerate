use actix_session::Session;
use actix_web::{Responder, web};
use log::debug;
use tracing_attributes::instrument;

use crate::models::configuration::Config;
use crate::utils::utils::redirect_to_page;

///
/// Logout
///
//#[instrument]
#[instrument(skip(session))]
pub async fn logout(
    session: Session,
    data: web::Data<Config>,
) -> impl Responder {
    let sign_out_url = format!(
        "{}?post_logout_redirect_uri={}",
        data.open_id_config
            .clone()
            .unwrap()
            .end_session_endpoint
            .unwrap(),
        urlencoding::encode(data.default_page.clone().as_str())
    );
    debug!("redirect to url > {}", sign_out_url);
    session.purge();
    debug!("Session was purged");
    redirect_to_page(sign_out_url.as_str())
}