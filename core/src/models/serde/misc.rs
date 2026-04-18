use crate::Result;
use crate::models::serde::{FromStateMap, strip_prefix};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventInfo {
    pub date: Option<String>,
    pub start_time: Option<String>,
    pub city: Option<String>,
    pub game_no: Option<String>,
    pub host_league: Option<String>,
    pub state: Option<String>,
    pub tournament: Option<String>,
    pub venue: Option<String>,
}

impl FromStateMap for EventInfo {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        let state = strip_prefix(state, "EventInfo.");
        log::debug!("{:?}", state);
        Ok(serde_path_to_error::deserialize(state)?)
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum LabelType {
    Replaced,
    Start,
    Stop,
    Timeout,
    Undo,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum PenaltyCode {
    #[serde(rename = "?")]
    QuestionMark,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    L,
    M,
    N,
    P,
    X,
}
