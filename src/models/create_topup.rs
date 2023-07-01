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
pub struct CreateTopup {
    #[serde(rename = "wallet_id")]
    pub wallet_id: String,
    #[serde(rename = "amount")]
    pub amount: i32,
}

impl CreateTopup {
    pub fn new(wallet_id: String, amount: i32) -> CreateTopup {
        CreateTopup {
            wallet_id,
            amount,
        }
    }
}

