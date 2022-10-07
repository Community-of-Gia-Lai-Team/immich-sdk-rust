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
pub struct GetAssetCountByTimeBucketDto {
    #[serde(rename = "timeGroup")]
    pub time_group: crate::models::TimeGroupEnum,
}

impl GetAssetCountByTimeBucketDto {
    pub fn new(time_group: crate::models::TimeGroupEnum) -> GetAssetCountByTimeBucketDto {
        GetAssetCountByTimeBucketDto {
            time_group,
        }
    }
}

