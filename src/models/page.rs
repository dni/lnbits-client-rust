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
pub struct Page {
    #[serde(rename = "data")]
    pub data: Vec<serde_json::Value>,
    #[serde(rename = "total")]
    pub total: i32,
}

impl Page {
    pub fn new(data: Vec<serde_json::Value>, total: i32) -> Page {
        Page {
            data,
            total,
        }
    }
}

