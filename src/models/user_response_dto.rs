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
pub struct UserResponseDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "profileImagePath")]
    pub profile_image_path: String,
    #[serde(rename = "shouldChangePassword")]
    pub should_change_password: bool,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
}

impl UserResponseDto {
    pub fn new(id: String, email: String, first_name: String, last_name: String, created_at: String, profile_image_path: String, should_change_password: bool, is_admin: bool) -> UserResponseDto {
        UserResponseDto {
            id,
            email,
            first_name,
            last_name,
            created_at,
            profile_image_path,
            should_change_password,
            is_admin,
        }
    }
}


