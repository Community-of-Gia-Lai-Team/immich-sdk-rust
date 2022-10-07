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
pub struct CuratedLocationsResponseDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "resizePath")]
    pub resize_path: String,
    #[serde(rename = "deviceAssetId")]
    pub device_asset_id: String,
    #[serde(rename = "deviceId")]
    pub device_id: String,
}

impl CuratedLocationsResponseDto {
    pub fn new(id: String, city: String, resize_path: String, device_asset_id: String, device_id: String) -> CuratedLocationsResponseDto {
        CuratedLocationsResponseDto {
            id,
            city,
            resize_path,
            device_asset_id,
            device_id,
        }
    }
}


