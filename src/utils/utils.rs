use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;
use jsonwebtoken::{Algorithm, decode, decode_header, DecodingKey, TokenData, Validation};
use jsonwebtoken::errors::{Error, ErrorKind};
use log::debug;
use serde::de::DeserializeOwned;
use tracing_attributes::instrument;

use crate::models::entra_id::{JWKS, JWKSKeyItem};

pub const KEY_ID_TOKEN: &'static str = "KEY_ID_TOKEN";

///
/// redirect to page
///
pub fn redirect_to_page(page: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, page))
        .finish()
}

///
/// Get JWKS Item by kid
///
#[instrument(level = "debug")]
fn get_jwks_item(jwks: &JWKS, kid: &str) -> Option<JWKSKeyItem> {
    for item in jwks.keys.iter() {
        let found_item = item
            .iter()
            .find(|&key| key.kid.clone().unwrap_or("".to_string()).eq(kid));
        if let Some(found_item) = found_item {
            return Some(found_item.clone());
        }
    }
    None
}

///
/// Validate JWT Token
///
//#[instrument]
#[instrument(level = "debug")]
pub fn jwt_token_validation<T>(jwt_token: &str,
                               jwks: &JWKS,
                               aud: Option<String>) -> Result<TokenData<T>, Error>
    where
        T: DeserializeOwned,
{
    debug!("JWT Token Validation");
    let header = decode_header(jwt_token);
    let mut validation = Validation::new(Algorithm::RS256);
    /*
    if entra_info.clone().issuer.is_some() {
        let issuer = entra_info.clone().issuer.unwrap();
        debug!("Issuer for validate : {}", issuer);
        validation.set_issuer(&[issuer.as_str()])
    }*/
    if aud.clone().is_some() {
        validation.set_audience(&[aud.clone().unwrap().as_str()]);
    }
    match header {
        Ok(h) => match get_jwks_item(jwks, h.kid.unwrap().as_str()) {
            Some(item) => {
                debug!("Found JWKS Item : {:#?}", item);
                let token = decode::<T>(
                    jwt_token,
                    &DecodingKey::from_rsa_components(
                        item.n.clone().unwrap().as_str(),
                        item.e.clone().unwrap().as_str(),
                    )
                        .unwrap(),
                    &validation,
                );
                debug!("Return token");
                token
            }
            None => Err(jsonwebtoken::errors::Error::from(
                ErrorKind::InvalidAudience,
            )),
        },
        Err(e) => Err(e),
    }
}