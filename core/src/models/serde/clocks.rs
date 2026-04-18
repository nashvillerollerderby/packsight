use crate::Result;
use crate::models::serde::{FromStateMap, JsDate};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum ClockType {
    Intermission,
    Jam,
    Lineup,
    Period,
    Timeout,
}

pub type Clocks = HashMap<ClockType, Clock>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Clock {
    pub direction: bool,
    pub id: String,
    pub clock_type: ClockType,
    #[serde(with = "serde_millis")]
    pub inverted_time: Duration,
    #[serde(with = "serde_millis")]
    pub maximum_time: Duration,
    pub name: String,
    pub number: u16,
    pub readonly: bool,
    pub running: bool,
    #[serde(deserialize_with = "chrono::serde::ts_milliseconds::deserialize")]
    #[serde(serialize_with = "chrono::serde::ts_milliseconds::serialize")]
    pub time: JsDate,
}

impl FromStateMap for Clocks {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        log::debug!("{:?}", state);
        Ok(state
            .iter()
            .fold(
                HashMap::new(),
                |mut acc: HashMap<ClockType, Map<String, Value>>, (k, v)| {
                    let (typ, key) = k.split_once(".").unwrap();
                    acc.entry(
                        serde_path_to_error::deserialize(Value::String(typ.to_string())).unwrap(),
                    )
                    .and_modify(|m| {
                        if key == "Id" {
                            let (uuid, clock_type) = v.as_str().unwrap().split_once("_").unwrap();
                            m.insert(key.into(), Value::String(uuid.into()));
                            m.insert("ClockType".into(), Value::String(clock_type.into()));
                        } else {
                            m.insert(key.into(), v.clone());
                        }
                    })
                    .or_insert_with(|| {
                        let mut m = Map::new();
                        m.insert(key.into(), v.clone());
                        m
                    });

                    log::debug!("{:?}", acc);

                    acc
                },
            )
            .into_iter()
            .map(|(k, v)| {
                log::debug!("{:?}: {:?}", k, v);
                (k, serde_path_to_error::deserialize(v).unwrap())
            })
            .collect())
    }
}
