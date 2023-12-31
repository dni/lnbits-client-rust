/*
 * lnbits
 *
 * API for LNbits, the free and open source bitcoin wallet and accounts system with plugins.
 *
 * The version of the OpenAPI document: 0.10.9
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateInvoiceData {
    #[serde(rename = "out", skip_serializing_if = "Option::is_none")]
    pub out: Option<bool>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(rename = "memo", skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "description_hash", skip_serializing_if = "Option::is_none")]
    pub description_hash: Option<String>,
    #[serde(rename = "unhashed_description", skip_serializing_if = "Option::is_none")]
    pub unhashed_description: Option<String>,
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<i32>,
    #[serde(rename = "lnurl_callback", skip_serializing_if = "Option::is_none")]
    pub lnurl_callback: Option<String>,
    #[serde(rename = "lnurl_balance_check", skip_serializing_if = "Option::is_none")]
    pub lnurl_balance_check: Option<String>,
    #[serde(rename = "extra", skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>,
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
    #[serde(rename = "internal", skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    #[serde(rename = "bolt11", skip_serializing_if = "Option::is_none")]
    pub bolt11: Option<String>,
}

impl CreateInvoiceData {
    pub fn new() -> CreateInvoiceData {
        CreateInvoiceData {
            out: None,
            amount: None,
            memo: None,
            unit: None,
            description_hash: None,
            unhashed_description: None,
            expiry: None,
            lnurl_callback: None,
            lnurl_balance_check: None,
            extra: None,
            webhook: None,
            internal: None,
            bolt11: None,
        }
    }
}


