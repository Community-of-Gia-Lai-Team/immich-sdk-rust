/*
 * Immich
 *
 * Immich API
 *
 * The version of the OpenAPI document: 1.17.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerPingResponse {
    #[serde(rename = "res")]
    pub res: String,
}

impl ServerPingResponse {
    pub fn new(res: String) -> ServerPingResponse {
        ServerPingResponse {
            res,
        }
    }
}


