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
pub struct UpdateDeviceInfoDto {
    #[serde(rename = "deviceType")]
    pub device_type: crate::models::DeviceTypeEnum,
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "isAutoBackup", skip_serializing_if = "Option::is_none")]
    pub is_auto_backup: Option<bool>,
}

impl UpdateDeviceInfoDto {
    pub fn new(device_type: crate::models::DeviceTypeEnum, device_id: String) -> UpdateDeviceInfoDto {
        UpdateDeviceInfoDto {
            device_type,
            device_id,
            is_auto_backup: None,
        }
    }
}


