use actix_session::Session;
use actix_web::{Responder, web};
use log::debug;
use oauth2::{AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, ResponseType, Scope};
use oauth2::basic::BasicClient;
use tracing_attributes::instrument;

use crate::models::configuration::Config;
use crate::utils::utils::redirect_to_page;

///
///  main page
///
//#[instrument]
#[instrument(skip(session))]
pub async fn login(
    session: Session,
    data: web::Data<Config>,
) -> impl Responder {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    debug!(
        "PKCE challenge : {:?} \r\n ,\
            PKCE verifier {:?}",
        pkce_challenge, pkce_verifier
    );

    let client = BasicClient::new(
        ClientId::new(data.client_id.clone()),
        Some(ClientSecret::new(data.client_secret.clone())),
        AuthUrl::new(
            data.open_id_config
                .clone()
                .unwrap()
                .authorization_endpoint
                .unwrap(),
        )
            .unwrap(),
        None,
    )
        // Set the URL the user will be redirected to after the authorization process.
        .set_redirect_uri(RedirectUrl::new(data.redirect.clone()).unwrap());

    let mut auth_req = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("offline_access".to_string()))
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_extra_param("nonce", "1234234233232322222")
        .set_pkce_challenge(pkce_challenge);


    auth_req = auth_req.add_extra_param("response_mode", "form_post");
    let res_type = ResponseType::new("id_token".to_string());
    auth_req = auth_req.set_response_type(&res_type);
    let (auth_url, csrf_token) = auth_req.url();

    debug!("csrf_token = {}", csrf_token.secret());
    let auth_url = format!("{}", auth_url);
    debug!("Url : {}", auth_url.clone());

    let s = session.insert(csrf_token.secret().as_str(), pkce_verifier);
    redirect_to_page(auth_url.as_str())
}
