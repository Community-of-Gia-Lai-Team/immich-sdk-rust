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
pub struct LoginCredentialDto {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl LoginCredentialDto {
    pub fn new(email: String, password: String) -> LoginCredentialDto {
        LoginCredentialDto {
            email,
            password,
        }
    }
}


