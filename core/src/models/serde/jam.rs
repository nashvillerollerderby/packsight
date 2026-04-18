use crate::error::Result;
use crate::models::serde::{FromStateMap, JsDate, get_ids, insert_base, strip_prefix};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use uuid::Uuid;

pub type Jams = HashMap<u16, Jam>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Jam {
    #[serde(with = "serde_millis")]
    pub duration: Duration,
    pub id: Uuid,
    pub injury_continuation: bool,
    pub next: Option<String>,
    pub number: u16,
    pub overtime: bool,
    pub penalty: HashMap<Uuid, Uuid>,
    #[serde(with = "serde_millis")]
    pub period_clock_display_end: Duration,
    #[serde(with = "serde_millis")]
    pub period_clock_elapsed_end: Duration,
    #[serde(with = "serde_millis")]
    pub period_clock_elapsed_start: Duration,
    pub period_number: u8,
    pub previous: Option<String>,
    pub readonly: bool,
    pub star_pass: bool,
    pub team_jam: (TeamJam, TeamJam),
    #[serde(deserialize_with = "chrono::serde::ts_milliseconds::deserialize")]
    #[serde(serialize_with = "chrono::serde::ts_milliseconds::serialize")]
    pub walltime_end: JsDate,
    #[serde(deserialize_with = "chrono::serde::ts_milliseconds::deserialize")]
    #[serde(serialize_with = "chrono::serde::ts_milliseconds::serialize")]
    pub walltime_start: JsDate,
}

impl FromStateMap for Jams {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        let mut result = HashMap::new();
        let jam_ids = get_ids(state, |i| i.parse::<u16>().unwrap());

        for id in jam_ids {
            let state = &strip_prefix(state, &format!("{}.", id));
            let mut jam = Map::new();
            insert_base(state, &mut jam, None);

            log::info!("{:?}", jam);

            let penalty = strip_prefix(state, "Penalty.");
            jam.insert("Penalty".to_string(), Value::Object(penalty));

            let mut team_jams = Vec::new();
            team_jams.push(serde_json::to_value(TeamJam::from_state_map(
                &strip_prefix(state, "TeamJam.1."),
            )?)?);
            team_jams.push(serde_json::to_value(TeamJam::from_state_map(
                &strip_prefix(state, "TeamJam.2."),
            )?)?);
            jam.insert("TeamJam".to_string(), Value::Array(team_jams));

            result.insert(id, serde_path_to_error::deserialize(jam)?);
        }

        Ok(result)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TeamJam {
    #[serde(rename = "AfterSPScore")]
    pub after_sp_score: u16,
    pub calloff: bool,
    pub current_trip: Uuid,
    pub current_trip_number: u16,
    pub display_lead: bool,
    pub fielding: Fielding,
    pub id: String,
    pub injury: bool,
    pub jam_score: u16,
    pub last_score: u16,
    pub lead: bool,
    pub lost: bool,
    pub next: Option<String>,
    pub no_initial: bool,
    pub no_pivot: bool,
    pub number: u16,
    pub os_offset: u16,
    pub os_offset_reason: String,
    pub previous: Option<String>,
    pub readonly: bool,
    pub scoring_trips: HashMap<u16, ScoringTrip>,
    pub star_pass: bool,
    pub total_score: u16,
}

impl FromStateMap for TeamJam {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        let mut result = Map::new();
        insert_base(state, &mut result, None);

        result.insert(
            "Fielding".to_string(),
            serde_json::to_value(Fielding::from_state_map(&strip_prefix(state, "Fielding."))?)?,
        );

        result.insert(
            "ScoringTrips".to_string(),
            serde_json::to_value(ScoringTrips::from_state_map(&strip_prefix(
                state,
                "ScoringTrip.",
            ))?)?,
        );

        log::debug!("{:?}", result);

        Ok(serde_path_to_error::deserialize(result)?)
    }
}

pub type ScoringTrips = HashMap<u16, ScoringTrip>;

impl FromStateMap for ScoringTrips {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        let mut result = HashMap::new();
        let ids = get_ids(state, |i| i.parse::<u16>().unwrap());
        for id in ids {
            let state = strip_prefix(state, &format!("{}.", id));
            let mut scoring_trip = Map::new();
            insert_base(&state, &mut scoring_trip, None);
            result.insert(id, serde_path_to_error::deserialize(scoring_trip)?);
        }

        Ok(result)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScoringTrip {
    #[serde(rename = "AfterSP")]
    pub after_sp: bool,
    pub annotation: String,
    pub current: bool,
    pub duration: u32,
    pub id: Uuid,
    pub jam_clock_end: u32,
    pub jam_clock_start: u32,
    pub next: Option<String>,
    pub number: u16,
    pub previous: Option<String>,
    pub readonly: bool,
    pub score: u16,
}

impl FromStateMap for ScoringTrip {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        let mut result = Map::new();
        insert_base(state, &mut result, None);
        Ok(serde_path_to_error::deserialize(result)?)
    }
}

pub type Fielding = HashMap<FieldingPosition, FieldingPositionSkater>;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "PascalCase")]
pub enum FieldingPosition {
    Blocker1,
    Blocker2,
    Blocker3,
    Jammer,
    Pivot,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FieldingPositionSkater {
    pub annotation: String,
    #[serde(rename = "BoxTripSymbolsAfterSP")]
    pub box_trip_symbols_after_sp: String,
    #[serde(rename = "BoxTripSymbolsBeforeSP")]
    pub box_trip_symbols_before_sp: String,
    pub box_trip_symbols: String,
    pub id: String,
    pub next: Option<String>,
    pub not_fielded: bool,
    pub number: u16,
    pub penalty_box: bool,
    pub position: String,
    pub previous: Option<String>,
    pub readonly: bool,
    pub sit_for_3: bool,
    pub skater: Option<Uuid>,
    pub skater_number: String,
}

impl FromStateMap for Fielding {
    fn from_state_map(state: &Map<String, Value>) -> crate::error::Result<Self> {
        let mut result = HashMap::new();

        let fielding_positions = state
            .keys()
            .map(|k| k.split_once(".").unwrap().0.to_string())
            .collect::<HashSet<String>>();

        for position in fielding_positions {
            let mut fielding_position_skater = Map::new();
            let state = strip_prefix(state, &format!("{}.", position));
            insert_base(&state, &mut fielding_position_skater, None);

            log::debug!("{:?}", fielding_position_skater);

            result.insert(
                serde_path_to_error::deserialize(Value::String(position))?,
                serde_path_to_error::deserialize(fielding_position_skater)?,
            );
        }

        Ok(result)
    }
}
