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
pub struct LoginResponseDto {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userEmail")]
    pub user_email: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    #[serde(rename = "shouldChangePassword")]
    pub should_change_password: bool,
}

impl LoginResponseDto {
    pub fn new(access_token: String, user_id: String, user_email: String, first_name: String, last_name: String, profile_image_path: String, is_admin: bool, should_change_password: bool) -> LoginResponseDto {
        LoginResponseDto {
            access_token,
            user_id,
            user_email,
            first_name,
            last_name,
            profile_image_path,
            is_admin,
            should_change_password,
        }
    }
}


