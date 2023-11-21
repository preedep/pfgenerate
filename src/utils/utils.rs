use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{Algorithm, decode, decode_header, DecodingKey, TokenData, Validation};
use log::debug;
use serde::de::DeserializeOwned;
use tracing_attributes::instrument;
use crate::models::entra_id::{JWKS, JWKSKeyItem};

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
pub fn jwt_token_validation<T>(jwt_token: &str, jwks: &JWKS) -> Result<TokenData<T>, Error>
    where
        T: DeserializeOwned,
{
    let header = decode_header(jwt_token);
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
                    &Validation::new(Algorithm::RS256),
                );
                token
            }
            None => Err(jsonwebtoken::errors::Error::from(
                ErrorKind::InvalidAudience,
            )),
        },
        Err(e) => Err(e),
    }
}