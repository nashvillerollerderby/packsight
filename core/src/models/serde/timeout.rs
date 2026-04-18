use crate::models::serde::{FromStateMap, JsDate, get_ids, insert_base, strip_prefix};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use uuid::Uuid;

pub type Timeouts = HashMap<Uuid, Timeout>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Timeout {
    pub duration: u16,
    pub id: Uuid,
    pub or_request: String,
    pub or_result: String,
    pub owner: String,
    pub period_clock_elapsed_end: u16,
    pub period_clock_elapsed_start: u16,
    pub period_clock_end: u16,
    pub preceding_jam: Uuid,
    pub preceding_jam_number: u16,
    pub readonly: bool,
    pub retained_review: bool,
    pub review: bool,
    pub running: bool,
    #[serde(deserialize_with = "chrono::serde::ts_milliseconds::deserialize")]
    #[serde(serialize_with = "chrono::serde::ts_milliseconds::serialize")]
    pub walltime_end: JsDate,
    #[serde(deserialize_with = "chrono::serde::ts_milliseconds::deserialize")]
    #[serde(serialize_with = "chrono::serde::ts_milliseconds::serialize")]
    pub walltime_start: JsDate,
}

impl FromStateMap for Timeouts {
    fn from_state_map(state: &Map<String, Value>) -> crate::error::Result<Self> {
        let mut result = HashMap::new();
        let ids = get_ids(state, |i: &str| {
            serde_path_to_error::deserialize(Value::String(i.to_string())).unwrap()
        });

        for id in ids {
            let state = strip_prefix(&state, &format!("{}.", id));
            let mut timeout = Map::new();
            insert_base(&state, &mut timeout, None);

            result.insert(id, serde_path_to_error::deserialize(timeout)?);
        }

        Ok(result)
    }
}
