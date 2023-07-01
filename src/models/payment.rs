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
pub struct Payment {
    #[serde(rename = "checking_id")]
    pub checking_id: String,
    #[serde(rename = "pending")]
    pub pending: bool,
    #[serde(rename = "amount")]
    pub amount: i32,
    #[serde(rename = "fee")]
    pub fee: i32,
    #[serde(rename = "memo", skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(rename = "time")]
    pub time: i32,
    #[serde(rename = "bolt11")]
    pub bolt11: String,
    #[serde(rename = "preimage")]
    pub preimage: String,
    #[serde(rename = "payment_hash")]
    pub payment_hash: String,
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<f32>,
    #[serde(rename = "extra", skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>,
    #[serde(rename = "wallet_id")]
    pub wallet_id: String,
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<String>,
    #[serde(rename = "webhook_status", skip_serializing_if = "Option::is_none")]
    pub webhook_status: Option<i32>,
}

impl Payment {
    pub fn new(checking_id: String, pending: bool, amount: i32, fee: i32, time: i32, bolt11: String, preimage: String, payment_hash: String, wallet_id: String) -> Payment {
        Payment {
            checking_id,
            pending,
            amount,
            fee,
            memo: None,
            time,
            bolt11,
            preimage,
            payment_hash,
            expiry: None,
            extra: None,
            wallet_id,
            webhook: None,
            webhook_status: None,
        }
    }
}


