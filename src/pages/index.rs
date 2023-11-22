use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use handlebars::Handlebars;
use log::debug;
use tracing_attributes::instrument;

use crate::models::configuration::Config;
use crate::models::entra_id::IDToken;
use crate::utils::utils::KEY_ID_TOKEN;

#[instrument(skip(session))]
pub async fn page_index(
    session: Session,
    data: web::Data<Config>,
    hb: web::Data<Handlebars<'_>>,
) -> impl Responder {
    match session.get::<IDToken>(KEY_ID_TOKEN).unwrap() {
        None => {}
        Some(id_token) => {
            debug!("Page Index with ID token {:#?}", id_token);
        }
    }
    let x = IDToken{
        aud: None,
        iss: None,
        iat: None,
        nbf: None,
        exp: None,
        acct: None,
        acrs: None,
        aio: None,
        auth_time: None,
        ctry: None,
        email: None,
        family_name: None,
        given_name: None,
        groups: None,
        idp: None,
        ipaddr: None,
        login_hint: None,
        name: None,
        nonce: None,
        oid: None,
        preferred_username: None,
        rh: None,
        sid: None,
        sub: None,
        tenant_ctry: None,
        tenant_region_scope: None,
        tid: None,
        uti: None,
        ver: None,
        wids: None,
        xms_pl: None,
        xms_tpl: None,
        employee_id: None,
        department: None,
        companyname: None,
        officelocation: None,
    };
    let body = hb.render("index", &x).unwrap();
    HttpResponse::Ok().body(body)
}