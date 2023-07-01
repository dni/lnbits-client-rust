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
pub struct ValidationError {
    #[serde(rename = "loc")]
    pub loc: Vec<crate::models::LocationInner>,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl ValidationError {
    pub fn new(loc: Vec<crate::models::LocationInner>, msg: String, r#type: String) -> ValidationError {
        ValidationError {
            loc,
            msg,
            r#type,
        }
    }
}


