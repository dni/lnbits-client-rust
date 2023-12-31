/*
 * lnbits
 *
 * API for LNbits, the free and open source bitcoin wallet and accounts system with plugins.
 *
 * The version of the OpenAPI document: 0.10.9
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EditableSettings : Base class for settings, allowing values to be overridden by environment variables.  This is useful in production for secrets you do not wish to save in code, it plays nicely with docker(-compose), Heroku and any 12 factor app design.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EditableSettings {
    #[serde(rename = "lightning_invoice_expiry", skip_serializing_if = "Option::is_none")]
    pub lightning_invoice_expiry: Option<i32>,
    #[serde(rename = "boltz_network", skip_serializing_if = "Option::is_none")]
    pub boltz_network: Option<String>,
    #[serde(rename = "boltz_url", skip_serializing_if = "Option::is_none")]
    pub boltz_url: Option<String>,
    #[serde(rename = "boltz_mempool_space_url", skip_serializing_if = "Option::is_none")]
    pub boltz_mempool_space_url: Option<String>,
    #[serde(rename = "boltz_mempool_space_url_ws", skip_serializing_if = "Option::is_none")]
    pub boltz_mempool_space_url_ws: Option<String>,
    #[serde(rename = "lntips_api_endpoint", skip_serializing_if = "Option::is_none")]
    pub lntips_api_endpoint: Option<String>,
    #[serde(rename = "lntips_api_key", skip_serializing_if = "Option::is_none")]
    pub lntips_api_key: Option<String>,
    #[serde(rename = "lntips_admin_key", skip_serializing_if = "Option::is_none")]
    pub lntips_admin_key: Option<String>,
    #[serde(rename = "lntips_invoice_key", skip_serializing_if = "Option::is_none")]
    pub lntips_invoice_key: Option<String>,
    #[serde(rename = "spark_url", skip_serializing_if = "Option::is_none")]
    pub spark_url: Option<String>,
    #[serde(rename = "spark_token", skip_serializing_if = "Option::is_none")]
    pub spark_token: Option<String>,
    #[serde(rename = "opennode_api_endpoint", skip_serializing_if = "Option::is_none")]
    pub opennode_api_endpoint: Option<String>,
    #[serde(rename = "opennode_key", skip_serializing_if = "Option::is_none")]
    pub opennode_key: Option<String>,
    #[serde(rename = "opennode_admin_key", skip_serializing_if = "Option::is_none")]
    pub opennode_admin_key: Option<String>,
    #[serde(rename = "opennode_invoice_key", skip_serializing_if = "Option::is_none")]
    pub opennode_invoice_key: Option<String>,
    #[serde(rename = "lnpay_api_endpoint", skip_serializing_if = "Option::is_none")]
    pub lnpay_api_endpoint: Option<String>,
    #[serde(rename = "lnpay_api_key", skip_serializing_if = "Option::is_none")]
    pub lnpay_api_key: Option<String>,
    #[serde(rename = "lnpay_wallet_key", skip_serializing_if = "Option::is_none")]
    pub lnpay_wallet_key: Option<String>,
    #[serde(rename = "lnpay_admin_key", skip_serializing_if = "Option::is_none")]
    pub lnpay_admin_key: Option<String>,
    #[serde(rename = "lnd_grpc_endpoint", skip_serializing_if = "Option::is_none")]
    pub lnd_grpc_endpoint: Option<String>,
    #[serde(rename = "lnd_grpc_cert", skip_serializing_if = "Option::is_none")]
    pub lnd_grpc_cert: Option<String>,
    #[serde(rename = "lnd_grpc_port", skip_serializing_if = "Option::is_none")]
    pub lnd_grpc_port: Option<i32>,
    #[serde(rename = "lnd_grpc_admin_macaroon", skip_serializing_if = "Option::is_none")]
    pub lnd_grpc_admin_macaroon: Option<String>,
    #[serde(rename = "lnd_grpc_invoice_macaroon", skip_serializing_if = "Option::is_none")]
    pub lnd_grpc_invoice_macaroon: Option<String>,
    #[serde(rename = "lnd_grpc_macaroon", skip_serializing_if = "Option::is_none")]
    pub lnd_grpc_macaroon: Option<String>,
    #[serde(rename = "lnd_grpc_macaroon_encrypted", skip_serializing_if = "Option::is_none")]
    pub lnd_grpc_macaroon_encrypted: Option<String>,
    #[serde(rename = "lnd_rest_endpoint", skip_serializing_if = "Option::is_none")]
    pub lnd_rest_endpoint: Option<String>,
    #[serde(rename = "lnd_rest_cert", skip_serializing_if = "Option::is_none")]
    pub lnd_rest_cert: Option<String>,
    #[serde(rename = "lnd_rest_macaroon", skip_serializing_if = "Option::is_none")]
    pub lnd_rest_macaroon: Option<String>,
    #[serde(rename = "lnd_rest_macaroon_encrypted", skip_serializing_if = "Option::is_none")]
    pub lnd_rest_macaroon_encrypted: Option<String>,
    #[serde(rename = "lnd_cert", skip_serializing_if = "Option::is_none")]
    pub lnd_cert: Option<String>,
    #[serde(rename = "lnd_admin_macaroon", skip_serializing_if = "Option::is_none")]
    pub lnd_admin_macaroon: Option<String>,
    #[serde(rename = "lnd_invoice_macaroon", skip_serializing_if = "Option::is_none")]
    pub lnd_invoice_macaroon: Option<String>,
    #[serde(rename = "lnd_rest_admin_macaroon", skip_serializing_if = "Option::is_none")]
    pub lnd_rest_admin_macaroon: Option<String>,
    #[serde(rename = "lnd_rest_invoice_macaroon", skip_serializing_if = "Option::is_none")]
    pub lnd_rest_invoice_macaroon: Option<String>,
    #[serde(rename = "eclair_url", skip_serializing_if = "Option::is_none")]
    pub eclair_url: Option<String>,
    #[serde(rename = "eclair_pass", skip_serializing_if = "Option::is_none")]
    pub eclair_pass: Option<String>,
    #[serde(rename = "corelightning_rpc", skip_serializing_if = "Option::is_none")]
    pub corelightning_rpc: Option<String>,
    #[serde(rename = "clightning_rpc", skip_serializing_if = "Option::is_none")]
    pub clightning_rpc: Option<String>,
    #[serde(rename = "cliche_endpoint", skip_serializing_if = "Option::is_none")]
    pub cliche_endpoint: Option<String>,
    #[serde(rename = "lnbits_endpoint", skip_serializing_if = "Option::is_none")]
    pub lnbits_endpoint: Option<String>,
    #[serde(rename = "lnbits_key", skip_serializing_if = "Option::is_none")]
    pub lnbits_key: Option<String>,
    #[serde(rename = "lnbits_admin_key", skip_serializing_if = "Option::is_none")]
    pub lnbits_admin_key: Option<String>,
    #[serde(rename = "lnbits_invoice_key", skip_serializing_if = "Option::is_none")]
    pub lnbits_invoice_key: Option<String>,
    #[serde(rename = "fake_wallet_secret", skip_serializing_if = "Option::is_none")]
    pub fake_wallet_secret: Option<String>,
    #[serde(rename = "lnbits_backend_wallet_class", skip_serializing_if = "Option::is_none")]
    pub lnbits_backend_wallet_class: Option<String>,
    #[serde(rename = "lnbits_rate_limit_no", skip_serializing_if = "Option::is_none")]
    pub lnbits_rate_limit_no: Option<String>,
    #[serde(rename = "lnbits_rate_limit_unit", skip_serializing_if = "Option::is_none")]
    pub lnbits_rate_limit_unit: Option<String>,
    #[serde(rename = "lnbits_allowed_ips", skip_serializing_if = "Option::is_none")]
    pub lnbits_allowed_ips: Option<Vec<String>>,
    #[serde(rename = "lnbits_blocked_ips", skip_serializing_if = "Option::is_none")]
    pub lnbits_blocked_ips: Option<Vec<String>>,
    #[serde(rename = "lnbits_notifications", skip_serializing_if = "Option::is_none")]
    pub lnbits_notifications: Option<bool>,
    #[serde(rename = "lnbits_killswitch", skip_serializing_if = "Option::is_none")]
    pub lnbits_killswitch: Option<bool>,
    #[serde(rename = "lnbits_killswitch_interval", skip_serializing_if = "Option::is_none")]
    pub lnbits_killswitch_interval: Option<i32>,
    #[serde(rename = "lnbits_watchdog", skip_serializing_if = "Option::is_none")]
    pub lnbits_watchdog: Option<bool>,
    #[serde(rename = "lnbits_watchdog_interval", skip_serializing_if = "Option::is_none")]
    pub lnbits_watchdog_interval: Option<i32>,
    #[serde(rename = "lnbits_watchdog_delta", skip_serializing_if = "Option::is_none")]
    pub lnbits_watchdog_delta: Option<i32>,
    #[serde(rename = "lnbits_status_manifest", skip_serializing_if = "Option::is_none")]
    pub lnbits_status_manifest: Option<String>,
    #[serde(rename = "lnbits_baseurl", skip_serializing_if = "Option::is_none")]
    pub lnbits_baseurl: Option<String>,
    #[serde(rename = "lnbits_reserve_fee_min", skip_serializing_if = "Option::is_none")]
    pub lnbits_reserve_fee_min: Option<i32>,
    #[serde(rename = "lnbits_reserve_fee_percent", skip_serializing_if = "Option::is_none")]
    pub lnbits_reserve_fee_percent: Option<f32>,
    #[serde(rename = "lnbits_service_fee", skip_serializing_if = "Option::is_none")]
    pub lnbits_service_fee: Option<f32>,
    #[serde(rename = "lnbits_hide_api", skip_serializing_if = "Option::is_none")]
    pub lnbits_hide_api: Option<bool>,
    #[serde(rename = "lnbits_denomination", skip_serializing_if = "Option::is_none")]
    pub lnbits_denomination: Option<String>,
    #[serde(rename = "lnbits_site_title", skip_serializing_if = "Option::is_none")]
    pub lnbits_site_title: Option<String>,
    #[serde(rename = "lnbits_site_tagline", skip_serializing_if = "Option::is_none")]
    pub lnbits_site_tagline: Option<String>,
    #[serde(rename = "lnbits_site_description", skip_serializing_if = "Option::is_none")]
    pub lnbits_site_description: Option<String>,
    #[serde(rename = "lnbits_default_wallet_name", skip_serializing_if = "Option::is_none")]
    pub lnbits_default_wallet_name: Option<String>,
    #[serde(rename = "lnbits_theme_options", skip_serializing_if = "Option::is_none")]
    pub lnbits_theme_options: Option<Vec<String>>,
    #[serde(rename = "lnbits_custom_logo", skip_serializing_if = "Option::is_none")]
    pub lnbits_custom_logo: Option<String>,
    #[serde(rename = "lnbits_ad_space_title", skip_serializing_if = "Option::is_none")]
    pub lnbits_ad_space_title: Option<String>,
    #[serde(rename = "lnbits_ad_space", skip_serializing_if = "Option::is_none")]
    pub lnbits_ad_space: Option<String>,
    #[serde(rename = "lnbits_ad_space_enabled", skip_serializing_if = "Option::is_none")]
    pub lnbits_ad_space_enabled: Option<bool>,
    #[serde(rename = "lnbits_allowed_currencies", skip_serializing_if = "Option::is_none")]
    pub lnbits_allowed_currencies: Option<Vec<String>>,
    #[serde(rename = "lnbits_admin_extensions", skip_serializing_if = "Option::is_none")]
    pub lnbits_admin_extensions: Option<Vec<String>>,
    #[serde(rename = "lnbits_extensions_manifests", skip_serializing_if = "Option::is_none")]
    pub lnbits_extensions_manifests: Option<Vec<String>>,
    #[serde(rename = "lnbits_admin_users", skip_serializing_if = "Option::is_none")]
    pub lnbits_admin_users: Option<Vec<String>>,
    #[serde(rename = "lnbits_allowed_users", skip_serializing_if = "Option::is_none")]
    pub lnbits_allowed_users: Option<Vec<String>>,
}

impl EditableSettings {
    /// Base class for settings, allowing values to be overridden by environment variables.  This is useful in production for secrets you do not wish to save in code, it plays nicely with docker(-compose), Heroku and any 12 factor app design.
    pub fn new() -> EditableSettings {
        EditableSettings {
            lightning_invoice_expiry: None,
            boltz_network: None,
            boltz_url: None,
            boltz_mempool_space_url: None,
            boltz_mempool_space_url_ws: None,
            lntips_api_endpoint: None,
            lntips_api_key: None,
            lntips_admin_key: None,
            lntips_invoice_key: None,
            spark_url: None,
            spark_token: None,
            opennode_api_endpoint: None,
            opennode_key: None,
            opennode_admin_key: None,
            opennode_invoice_key: None,
            lnpay_api_endpoint: None,
            lnpay_api_key: None,
            lnpay_wallet_key: None,
            lnpay_admin_key: None,
            lnd_grpc_endpoint: None,
            lnd_grpc_cert: None,
            lnd_grpc_port: None,
            lnd_grpc_admin_macaroon: None,
            lnd_grpc_invoice_macaroon: None,
            lnd_grpc_macaroon: None,
            lnd_grpc_macaroon_encrypted: None,
            lnd_rest_endpoint: None,
            lnd_rest_cert: None,
            lnd_rest_macaroon: None,
            lnd_rest_macaroon_encrypted: None,
            lnd_cert: None,
            lnd_admin_macaroon: None,
            lnd_invoice_macaroon: None,
            lnd_rest_admin_macaroon: None,
            lnd_rest_invoice_macaroon: None,
            eclair_url: None,
            eclair_pass: None,
            corelightning_rpc: None,
            clightning_rpc: None,
            cliche_endpoint: None,
            lnbits_endpoint: None,
            lnbits_key: None,
            lnbits_admin_key: None,
            lnbits_invoice_key: None,
            fake_wallet_secret: None,
            lnbits_backend_wallet_class: None,
            lnbits_rate_limit_no: None,
            lnbits_rate_limit_unit: None,
            lnbits_allowed_ips: None,
            lnbits_blocked_ips: None,
            lnbits_notifications: None,
            lnbits_killswitch: None,
            lnbits_killswitch_interval: None,
            lnbits_watchdog: None,
            lnbits_watchdog_interval: None,
            lnbits_watchdog_delta: None,
            lnbits_status_manifest: None,
            lnbits_baseurl: None,
            lnbits_reserve_fee_min: None,
            lnbits_reserve_fee_percent: None,
            lnbits_service_fee: None,
            lnbits_hide_api: None,
            lnbits_denomination: None,
            lnbits_site_title: None,
            lnbits_site_tagline: None,
            lnbits_site_description: None,
            lnbits_default_wallet_name: None,
            lnbits_theme_options: None,
            lnbits_custom_logo: None,
            lnbits_ad_space_title: None,
            lnbits_ad_space: None,
            lnbits_ad_space_enabled: None,
            lnbits_allowed_currencies: None,
            lnbits_admin_extensions: None,
            lnbits_extensions_manifests: None,
            lnbits_admin_users: None,
            lnbits_allowed_users: None,
        }
    }
}


