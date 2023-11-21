use actix_session::Session;
use actix_web::{Responder, web};
use log::{debug, error};
use oauth2::PkceCodeVerifier;
use tracing_attributes::instrument;

use crate::models::configuration::Config;
use crate::models::entra_id::{IDToken, ResponseAuthorized};
use crate::utils::utils::{jwt_token_validation, redirect_to_page};

///
///  main page
///
//#[instrument]
#[instrument(skip(session))]
pub async fn callback(
    session: Session,
    params: web::Form<ResponseAuthorized>,
    data: web::Data<Config>,
) -> impl Responder {
    debug!("Callback called = {:#?}", params);
    match params.0.state {
        None => {}
        Some(state) => {
            //Get Code for Verification from session
            let value = session.get::<PkceCodeVerifier>(state.as_str());
            match value {
                Ok(key) => {
                    if !(key.map_or(String::new(), |pkce| {
                        pkce.secret().to_string()
                    }).is_empty()) {
                        // Verify token with JWKS
                        let result = jwt_token_validation::<IDToken>(
                            &params.0.id_token.clone().unwrap(),
                            &data.jwks.clone().unwrap(),
                        );
                        match result {
                            Ok(result) => {
                                debug!("IDToken validation succeeded {:#?}", result);
                            }
                            Err(err) => {
                                error!("JWT Token Validate {}", err);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Get State Error {}", e);
                }
            }
        }
    }
    redirect_to_page("/pagerouting")
}
