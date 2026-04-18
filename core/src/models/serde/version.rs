use crate::error::Result;
use crate::models::serde::{FromStateMap, ScoreboardVersion};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    #[serde(rename = "release")]
    pub release: ScoreboardVersion,
    #[serde(rename = "release.user")]
    pub release_user: String,
    #[serde(rename = "release.commit")]
    pub release_commit: String,
    #[serde(rename = "release.host")]
    pub release_host: String,
    #[serde(rename = "release.time")]
    pub release_time: String,
}

impl FromStateMap for Version {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        let version_state = state
            .into_iter()
            .filter_map(|(k, v)| {
                if let Some(k) = k.strip_prefix("ScoreBoard.Version.") {
                    Some((k.to_string(), v.clone()))
                } else {
                    None
                }
            })
            .collect::<Map<String, Value>>();

        log::debug!("{:?}", version_state);

        Ok(serde_path_to_error::deserialize(version_state)?)
    }
}
