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
pub struct AssetCountByTimeBucket {
    #[serde(rename = "timeBucket")]
    pub time_bucket: String,
    #[serde(rename = "count")]
    pub count: i32,
}

impl AssetCountByTimeBucket {
    pub fn new(time_bucket: String, count: i32) -> AssetCountByTimeBucket {
        AssetCountByTimeBucket {
            time_bucket,
            count,
        }
    }
}


