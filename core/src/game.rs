// mod scoreboard;
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;
use std::io::Read;
use std::str::FromStr;
use std::sync::Arc;
use std::usize;
use std::time::Duration;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{MapAccess, Visitor};
use serde_json::{Value, Map};
use crate::error::{Error, Result};
use chrono::{DateTime, Utc};
use uuid::Uuid;

type JsDate = DateTime<Utc>;

pub trait FromStateMap {
    fn from_state_map(state: &Map<String, Value>) -> Self;
}

#[derive(Debug, Serialize)]
pub struct GameFileState {
    games: HashMap<uuid::Uuid, Game>,
    version: HashMap<VersionKey, String>,
}

impl GameFileState {
    fn new() -> Self {
        Self {
            games: HashMap::new(),
            version: HashMap::new(),
        }
    }
}

impl TryFrom<String> for GameFileState {
    type Error = crate::error::Error;

    fn try_from(value: String) -> Result<Self> {
        let map_from_string: Map<String, Value> = Map::from_str(&value)?;
        let state = map_from_string.get("state").unwrap().as_object().unwrap().clone();
        let mut game_file_state = Map::new();
        for (k, v) in state.into_iter() {
            let no_left_paren = k.replacen('(', ".", usize::MAX);
            let no_right_paren = no_left_paren.replacen(')', "", usize::MAX);
            log::info!("{}", no_right_paren);
            game_file_state.insert(no_right_paren, v.clone());
        }

        let games = Games::from_state_map(&game_file_state);
        let version = Version::from_state_map(&game_file_state);

        Ok(GameFileState {
            games,
            version
        }  ) 
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all(deserialize = "PascalCase"))]
enum ClockType {
    Intermission,
    Jam,
    Lineup,
    Period,
    Timeout,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all(deserialize = "PascalCase"))]
enum LabelType {
    Replaced,
    Start,
    Stop,
    Timeout,
    Undo
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all(deserialize = "PascalCase"))]
enum PenaltyCode {
    #[serde(rename="?")]
    Questionmark,
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
    X
}

type Clocks = HashMap<ClockType, Clock>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct Clock {
    direction: bool,
    id: Uuid,
    inverted_time: u32,
    maximum_time: u32,
    name: String,
    number: u16,
    readonly: bool,
    running: bool,
    time: u32,
}

impl FromStateMap for Clocks {
    fn from_state_map(state: &Map<String, Value>) -> Self {
        let mut result = HashMap::new();

        log::info!("{:?}", state);

        result
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all(deserialize = "PascalCase"))]
enum FieldingPosition {
    Blocker1,
    Blocker2,
    Blocker3,
    Jammer,
    Pivot
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct FieldingPositionSkater {
    annotation: String,
    box_trip_symbols: String,
    box_trip_after_sp: String,
    box_trip_before_sp: String,
    id: Uuid,
    next: Uuid,
    not_fielded: bool,
    number: u16,
    penalty_box: bool,
    position: Uuid,
    previous: Uuid,
    readonly: bool,
    sit_for_3: bool,
    skater: Uuid,
    skater_number: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct ScoringTrip {
    after_sp: bool,
    annotation: String,
    current: bool,
    duration: u16,
    id: Uuid,
    jam_clock_end: u16,
    jam_clock_start: u16,
    next: Uuid,
    number: u16,
    readonly: bool,
    score: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct TeamJam {
    after_sp_score: u16,
    calloff: bool,
    current_trip: Uuid,
    current_trip_number: u16,
    display_lead: bool,
    fielding: HashMap<FieldingPosition, FieldingPositionSkater>,
    id: Uuid,
    injury: bool,
    jam_score: u16,
    last_score: u16,
    lead: bool,
    lost: bool,
    next: Uuid,
    no_initial: bool,
    no_pivot: bool,
    number: u16,
    os_offset: u16,
    os_offset_reason: String,
    previous: Uuid,
    readonly: bool,
    scoring_trips: HashMap<u16, ScoringTrip>,
    star_pass: bool,
    total_score: u16
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct Jam {
    duration: u16,
    id: Uuid,
    injury_continuation: bool,
    next: Uuid,
    number: u16,
    overtime: bool,
    penalty: HashMap<Uuid, Uuid>,
    period_clock_display_end: u16,
    period_clock_elapsed_end: u16,
    period_clock_elapsed_start: u16,
    period_number: u8,
    previous: Uuid,
    readonly: bool,
    star_pass: bool,
    team_jam: (TeamJam, TeamJam),
    walltime_end: JsDate,
    walltime_start: JsDate
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct Timeout {
    duration: u16,
    id: Uuid,
    or_request: String,
    or_result: String,
    owner: String,
    period_clock_elapsed_end: u16,
    period_clock_elapsed_start: u16,
    period_clock_end: u16,
    preceding_jam: Uuid,
    preceding_jam_number: u16,
    readonly: bool,
    retained_review: bool,
    review: bool,
    running: bool,
    walltime_end: JsDate,
    walltime_start: JsDate
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct Period {
    current_jam: uuid::Uuid,
    current_jam_number: u16,
    duration: u16,
    id: Uuid,
    jams: HashMap<u16, Jam>,
    number: u8,
    previous: uuid::Uuid,
    readonly: bool,
    running: bool,
    sudden_scoring: bool,
    team1_penalty_count: u16,
    team1_points: u16,
    team2_penalty_count: u16,
    team2_points: u16,
    timeouts: HashMap<Uuid, Timeout>,
    walltime_end: JsDate,
    walltime_start: JsDate
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct Team {

}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct EventInfo {
    date: String,
    start_time: String
}

type Games = HashMap<Uuid, Game>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
struct Game {
    abort_reason: String,
    clocks: HashMap<ClockType, Clock>,
    clock_during_final_score: bool,
    current_period: uuid::Uuid,
    current_period_number: u8,
    current_timeout: String,
    event_info: EventInfo,
    export_blocked_by: String,
    filename: String,
    id: uuid::Uuid,
    in_jam: bool,
    in_overtime: bool,
    in_period: bool,
    in_sudden_scoring: bool,
    injury_continuation_upcoming: bool,
    // jam
    json_exists: bool,
    labels: HashMap<LabelType, String>,
    name: String,
    name_format: String,
    no_more_jam: bool,
    official_review: bool,
    official_score: bool,
    penalty_code: HashMap<PenaltyCode, String>,
    periods: HashMap<u8, Period>,
    read_only: bool,
    rule: HashMap<String, String>,
    ruleset_name: String,
    state: String,
    statsbook_exists: bool,
    suspensions_served: String,
    teams: HashMap<u8, Team>,
    timeout_owner: String,
    upcoming_jam: uuid::Uuid,
    upcoming_jam_number: u8,
    update_in_progress: bool,
}

impl FromStateMap for Game {
    fn from_state_map(state: &Map<String, Value>) -> Self {
        let mut result: Map<String, Value> = state.iter().filter_map(|(k, v)| {
            if k.split(".").collect::<Vec<&str>>().len() == 1 {
                Some((k.clone(), v.clone()))
            } else {
                None
            }
        }).collect();

        let clocks = Clocks::from_state_map(&state.iter().filter_map(|(k, v)| {
            if let Some(k) = k.strip_prefix("Clock.") {
                Some((k.to_string(), v.clone()))
            } else {
                None
            }
        }).collect::<Map<String, Value>>());
        result.insert("Clocks".to_string(), serde_json::to_value(clocks).unwrap());

        log::info!("{:?}", result);
z
        serde_json::from_value(serde_json::Value::Object(result)).unwrap()
    }
}

impl FromStateMap for Games {
    fn from_state_map(state: &Map<String, Value>) -> Self {
        let mut result = HashMap::new();

        let games_set = state.keys().filter_map(|k| {
            if k.contains("ScoreBoard.Game") {
                let uuid_str = k.split(".").nth(2).unwrap();
                Some(Uuid::from_str(uuid_str).unwrap())
            } else {
                None
            }
        }).collect::<HashSet<Uuid>>();

        for game_id in games_set {
            let game_prefix = format!("ScoreBoard.Game.{}.", game_id);
            let game = Map::new();
            let game_state = state.iter().filter_map(|(k, v)| {
                if let Some(k) = k.strip_prefix(&game_prefix) {
                    Some((k.to_string(), v.clone()))
                } else {
                    None
                }
            }).collect::<Map<String, Value>>();

            let game = Game::from_state_map(&game_state);

            result.insert(game_id, game);
        }

        return result;
    }
} 

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
enum VersionKey {
    #[serde(rename="release")]
    Release,
    #[serde(rename="release.commit")]
    Commit,
    #[serde(rename="release.host")]
    Host,
    #[serde(rename="release.time")]
    Time,
    #[serde(rename="release.user")]
    User
}

type Version = HashMap<VersionKey, String>;

impl FromStateMap for Version {
    fn from_state_map(state: &Map<String, Value>) -> Self {
        state.into_iter()
        .filter_map(|(k, v)| {
            if k.contains("ScoreBoard.Version") {
                let str = k.replace("ScoreBoard.Version.", "");
                match serde_json::from_value::<VersionKey>(serde_json::Value::String(str)) {
                    Ok(k) => {
                        Some((k, v.as_str().unwrap().to_string()))
                    }
                    Err(e) => {
                        log::error!("{}", e);
                        None
                    }
                }
            } else {
                None
            }
        }).collect::<HashMap<VersionKey, String>>()
    }
}