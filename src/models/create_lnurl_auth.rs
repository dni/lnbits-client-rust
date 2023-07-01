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
pub struct CreateLnurlAuth {
    #[serde(rename = "callback")]
    pub callback: String,
}

impl CreateLnurlAuth {
    pub fn new(callback: String) -> CreateLnurlAuth {
        CreateLnurlAuth {
            callback,
        }
    }
}

