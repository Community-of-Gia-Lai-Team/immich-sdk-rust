/*
 * Immich
 *
 * Immich API
 *
 * The version of the OpenAPI document: 1.17.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum JobCommand {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "stop")]
    Stop,

}

impl ToString for JobCommand {
    fn to_string(&self) -> String {
        match self {
            Self::Start => String::from("start"),
            Self::Stop => String::from("stop"),
        }
    }
}

impl Default for JobCommand {
    fn default() -> JobCommand {
        Self::Start
    }
}




