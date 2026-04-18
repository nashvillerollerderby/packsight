use crate::error::Result;
use crate::models::serde::{
    FromStateMap, Jams, JsDate, Timeouts, get_ids, insert_base, strip_prefix,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub type Periods = HashMap<u8, Period>;

impl FromStateMap for Periods {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        let period_ids = get_ids(state, |i| i.parse::<u8>().unwrap());
        log::debug!("PeriodIds {:?}", period_ids);
        let mut periods = HashMap::new();
        for id in period_ids {
            let state = strip_prefix(state, &format!("{}.", id));
            log::debug!("PeriodId {:?} {:?}", id, state);
            periods.insert(id, Period::from_state_map(&state)?);
        }

        Ok(periods)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Period {
    pub current_jam: Uuid,
    pub current_jam_number: u16,
    pub duration: u32,
    pub id: Uuid,
    pub jams: Jams,
    pub next: Option<String>,
    pub number: u8,
    pub previous: Option<String>,
    pub readonly: bool,
    pub running: bool,
    pub sudden_scoring: bool,
    pub team1_penalty_count: u16,
    pub team1_points: u16,
    pub team2_penalty_count: u16,
    pub team2_points: u16,
    pub timeouts: Timeouts,
    #[serde(deserialize_with = "chrono::serde::ts_milliseconds::deserialize")]
    #[serde(serialize_with = "chrono::serde::ts_milliseconds::serialize")]
    pub walltime_end: JsDate,
    #[serde(deserialize_with = "chrono::serde::ts_milliseconds::deserialize")]
    #[serde(serialize_with = "chrono::serde::ts_milliseconds::serialize")]
    pub walltime_start: JsDate,
}

impl FromStateMap for Period {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        let mut result = Map::new();
        insert_base(state, &mut result, None);

        let jams = Jams::from_state_map(&strip_prefix(state, "Jam."))?;
        result.insert("Jams".to_string(), serde_json::to_value(jams)?);

        let timeouts = Timeouts::from_state_map(&strip_prefix(state, "TimeOut."))?;
        result.insert("Timeouts".to_string(), serde_json::to_value(timeouts)?);

        log::info!("PeriodResult {:?}", result);

        Ok(serde_json::from_value(Value::Object(result))?)
    }
}
