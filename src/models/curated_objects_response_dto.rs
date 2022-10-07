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
pub struct CuratedObjectsResponseDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "resizePath")]
    pub resize_path: String,
    #[serde(rename = "deviceAssetId")]
    pub device_asset_id: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
}

impl CuratedObjectsResponseDto {
    pub fn new(id: String, object: String, resize_path: String, device_asset_id: String, device_id: String) -> CuratedObjectsResponseDto {
        CuratedObjectsResponseDto {
            id,
            object,
            resize_path,
            device_asset_id,
            device_id,
        }
    }
}


