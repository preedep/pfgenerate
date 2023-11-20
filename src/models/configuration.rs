use crate::models::entra_id::{JWKS, OpenIDConfigurationV2};

#[derive(Debug, Clone)]
pub struct Config {
    pub redis_url: String,
    pub redis_auth_key: String,
    pub tenant_id: String,
    pub default_page: String,
    pub redirect: String,
    pub client_id: String,
    pub client_secret: String,
    pub open_id_config: Option<OpenIDConfigurationV2>,
    pub jwks: Option<JWKS>,
}

impl Config {
    pub fn new(
        redis_url: String,
        redis_auth_key: String,
        tenant_id: String,
        default_page: String,
        redirect: String,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Config {
            redis_url,
            redis_auth_key,
            tenant_id,
            default_page,
            redirect,
            client_id,
            client_secret,
            open_id_config: None,
            jwks: None,
        }
    }
}