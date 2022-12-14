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
pub struct ValidateAccessTokenResponseDto {
    #[serde(rename = "authStatus")]
    pub auth_status: bool,
}

impl ValidateAccessTokenResponseDto {
    pub fn new(auth_status: bool) -> ValidateAccessTokenResponseDto {
        ValidateAccessTokenResponseDto {
            auth_status,
        }
    }
}


