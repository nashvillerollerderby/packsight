use crate::error::Result;
use crate::models::serde::{FromStateMap, JsDate, get_ids, insert_base, strip_prefix};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

pub type Teams = (Team, Team);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Team {
    pub active_score_adjustment_amount: i16,
    pub all_blockers_set: bool,
    pub alternate_name: AlternateName,
    pub box_trips: BoxTrips,
    pub calloff: bool,
    pub captain: Uuid,
    #[serde(rename = "Color.overlay.bg")]
    pub color: String,
    pub current_trip: Uuid,
    pub display_lead: bool,
    pub fielding_advance_pending: bool,
    pub file_name: String,
    pub full_name: String,
    pub id: String,
    pub in_official_review: bool,
    pub in_timeout: bool,
    pub initials: String,
    pub injury: bool,
    pub lead: bool,
    pub league_name: String,
    pub logo: String,
    pub lost: bool,
    pub name: String,
    pub no_initial: bool,
    pub no_pivot: bool,
    pub official_reviews: u8,
    pub prepared_team: String,
    pub prepared_team_connected: bool,
    pub readonly: bool,
    pub retained_official_review: bool,
    pub running_or_ended_team_jam: String,
    pub running_or_upcoming_team_jam: String,
    pub score: u16,
    pub skaters: Skaters,
    pub star_pass: bool,
    pub team_name: String,
    pub time_out: HashMap<Uuid, Uuid>,
    pub timeouts: u8,
    pub total_penalties: u8,
    pub trip_score: u16,
    pub uniform_color: Option<String>,
}

impl FromStateMap for Team {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        log::debug!("Team {:?}", state);

        let mut result = Map::new();
        insert_base(state, &mut result, None);
        result.insert(
            "Color.overlay.bg".to_string(),
            state.get("Color.overlay.bg").unwrap().clone(),
        );

        log::debug!("TeamResult {:?}", result);

        result.insert(
            "AlternateName".to_string(),
            Value::Object(Map::from_iter([
                (
                    "overlay".to_string(),
                    state.get("AlternateName.overlay").unwrap().clone(),
                ),
                (
                    "scoreboard".to_string(),
                    state.get("AlternateName.scoreboard").unwrap().clone(),
                ),
            ])),
        );

        let box_trips = BoxTrips::from_state_map(&strip_prefix(state, "BoxTrip."))?;
        result.insert("BoxTrips".to_string(), serde_json::to_value(box_trips)?);

        result.insert(
            "TimeOut".to_string(),
            Value::Object(strip_prefix(state, "TimeOut.")),
        );

        let skaters = Skaters::from_state_map(&strip_prefix(state, "Skater."))?;
        result.insert("Skaters".to_string(), serde_json::to_value(skaters)?);

        Ok(serde_path_to_error::deserialize(result)?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlternateName {
    pub overlay: String,
    pub scoreboard: String,
}

pub type BoxTrips = HashMap<Uuid, BoxTrip>;

impl FromStateMap for BoxTrips {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        log::debug!("BoxTrips {:?}", state);

        let mut result = HashMap::new();
        let ids = get_ids(state, |i| {
            serde_path_to_error::deserialize(Value::String(i.to_string())).unwrap()
        });

        for id in ids {
            result.insert(
                id,
                BoxTrip::from_state_map(&strip_prefix(state, &format!("{}.", id)))?,
            );
        }

        Ok(result)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BoxTrip {
    pub current_fielding: String,
    pub current_skater: String,
    pub duration: i32,
    #[serde(rename = "EndAfterSP")]
    pub end_after_sp: bool,
    pub end_between_jams: bool,
    pub end_fielding: Option<String>,
    pub end_jam_number: u16,
    pub fielding: HashMap<String, String>,
    pub id: Uuid,
    pub is_current: bool,
    pub jam_clock_end: i32,
    #[serde(with = "serde_millis")]
    pub jam_clock_start: Duration,
    pub penalties: HashMap<Uuid, Uuid>,
    pub penalty_codes: String,
    pub penalty_details: String,
    pub readonly: bool,
    pub roster_number: String,
    #[serde(with = "serde_millis")]
    pub shortened: Duration,
    #[serde(rename = "StartAfterSP")]
    pub start_after_sp: bool,
    pub start_between_jams: bool,
    pub start_fielding: String,
    pub start_jam_number: u16,
    pub timing_stopped: bool,
    pub total_penalties: u8,
    #[serde(deserialize_with = "chrono::serde::ts_milliseconds::deserialize")]
    #[serde(serialize_with = "chrono::serde::ts_milliseconds::serialize")]
    pub walltime_end: JsDate,
    #[serde(deserialize_with = "chrono::serde::ts_milliseconds::deserialize")]
    #[serde(serialize_with = "chrono::serde::ts_milliseconds::serialize")]
    pub walltime_start: JsDate,
}

impl FromStateMap for BoxTrip {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        log::debug!("BoxTrip {:?}", state);

        let mut result = Map::new();
        insert_base(state, &mut result, None);

        result.insert(
            "Fielding".to_string(),
            Value::Object(strip_prefix(state, "Fielding.")),
        );
        result.insert(
            "Penalties".to_string(),
            Value::Object(strip_prefix(state, "Penalty.")),
        );

        Ok(serde_path_to_error::deserialize(result)?)
    }
}

pub type Skaters = HashMap<Uuid, Skater>;

impl FromStateMap for Skaters {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        log::debug!("Skaters {:?}", state);

        let mut result = HashMap::new();
        let ids = get_ids(state, |i| {
            serde_path_to_error::deserialize(Value::String(i.to_string())).unwrap()
        });

        for id in ids {
            result.insert(
                id,
                Skater::from_state_map(&strip_prefix(state, &format!("{}.", id)))?,
            );
        }

        Ok(result)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Skater {
    pub base_role: String,
    pub color: String,
    pub current_box_symbols: String,
    pub current_penalties: String,
    #[serde(with = "serde_millis")]
    pub extra_penalty_time: Duration,
    pub fielding: HashMap<String, String>,
    pub flags: String,
    pub has_unserved: bool,
    pub id: Uuid,
    pub name: String,
    pub penalties: Penalties,
    pub penalty_box: bool,
    pub penalty_count: u8,
    pub penalty_details: String,
    pub prepared_skater: Uuid,
    pub pronouns: String,
    pub readonly: bool,
    pub role: String,
    pub roster_number: String,
}

impl FromStateMap for Skater {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        log::debug!("Skater {:?}", state);

        let mut result = Map::new();
        insert_base(state, &mut result, None);

        let mut fieldings = Map::new();
        {
            let state = strip_prefix(state, "Fielding.");
            insert_base(&state, &mut fieldings, None);
        }
        result.insert("Fielding".to_string(), Value::Object(fieldings));

        let penalties = Penalties::from_state_map(&strip_prefix(state, "Penalty."))?;
        result.insert("Penalties".to_string(), serde_json::to_value(penalties)?);

        Ok(serde_path_to_error::deserialize(result)?)
    }
}

pub type Penalties = HashMap<u8, Penalty>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Penalty {
    pub box_trip: Option<Uuid>,
    pub code: String,
    pub force_served: bool,
    pub id: Uuid,
    pub jam: Uuid,
    pub jam_number: u8,
    pub next: Option<Uuid>,
    pub number: u8,
    pub period_number: u8,
    pub previous: Option<Uuid>,
    pub readonly: bool,
    pub served: bool,
    pub serving: bool,
    #[serde(deserialize_with = "chrono::serde::ts_milliseconds::deserialize")]
    #[serde(serialize_with = "chrono::serde::ts_milliseconds::serialize")]
    pub time: JsDate,
}

impl FromStateMap for Penalties {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        log::debug!("Penalties {:?}", state);

        let mut result = HashMap::new();
        let penalty_ids = get_ids(state, |i| i.parse::<u8>().unwrap());

        for id in penalty_ids {
            let state = strip_prefix(state, &format!("{}.", id));
            let mut penalty = Map::new();
            insert_base(&state, &mut penalty, None);

            result.insert(id, serde_path_to_error::deserialize(penalty)?);
        }

        Ok(result)
    }
}
