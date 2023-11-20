use serde::{Deserialize, Serialize};

///
/// Open ID Configuration
///
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenIDConfigurationV2 {
    #[serde(rename = "token_endpoint")]
    pub token_endpoint: Option<String>,
    #[serde(rename = "token_endpoint_auth_methods_supported")]
    pub token_endpoint_auth_methods_supported: Option<Vec<String>>,
    #[serde(rename = "jwks_uri")]
    pub jwks_uri: Option<String>,
    #[serde(rename = "response_modes_supported")]
    pub response_modes_supported: Option<Vec<String>>,
    #[serde(rename = "subject_types_supported")]
    pub subject_types_supported: Option<Vec<String>>,
    #[serde(rename = "id_token_signing_alg_values_supported")]
    pub id_token_signing_alg_values_supported: Option<Vec<String>>,
    #[serde(rename = "response_types_supported")]
    pub response_types_supported: Option<Vec<String>>,
    #[serde(rename = "scopes_supported")]
    pub scopes_supported: Option<Vec<String>>,
    pub issuer: Option<String>,
    #[serde(rename = "request_uri_parameter_supported")]
    pub request_uri_parameter_supported: Option<bool>,
    #[serde(rename = "userinfo_endpoint")]
    pub userinfo_endpoint: Option<String>,
    #[serde(rename = "authorization_endpoint")]
    pub authorization_endpoint: Option<String>,
    #[serde(rename = "device_authorization_endpoint")]
    pub device_authorization_endpoint: Option<String>,
    #[serde(rename = "http_logout_supported")]
    pub http_logout_supported: Option<bool>,
    #[serde(rename = "frontchannel_logout_supported")]
    pub frontchannel_logout_supported: Option<bool>,
    #[serde(rename = "end_session_endpoint")]
    pub end_session_endpoint: Option<String>,
    #[serde(rename = "claims_supported")]
    pub claims_supported: Option<Vec<String>>,
    #[serde(rename = "kerberos_endpoint")]
    pub kerberos_endpoint: Option<String>,
    #[serde(rename = "tenant_region_scope")]
    pub tenant_region_scope: Option<String>,
    #[serde(rename = "cloud_instance_name")]
    pub cloud_instance_name: Option<String>,
    #[serde(rename = "cloud_graph_host_name")]
    pub cloud_graph_host_name: Option<String>,
    #[serde(rename = "msgraph_host")]
    pub msgraph_host: Option<String>,
    #[serde(rename = "rbac_url")]
    pub rbac_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JWKS {
    pub keys: Option<Vec<JWKSKeyItem>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JWKSKeyItem {
    pub kty: Option<String>,
    #[serde(rename = "use")]
    pub use_field: Option<String>,
    pub kid: Option<String>,
    pub x5t: Option<String>,
    pub n: Option<String>,
    pub e: Option<String>,
    pub x5c: Option<Vec<String>>,
    pub issuer: Option<String>,
}