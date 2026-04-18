use crate::models::serde::{
    Clock, ClockType, Clocks, EventInfo, FromStateMap, LabelType, PenaltyCode, Periods, Result,
    Team, Teams, insert_base, strip_prefix,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use uuid::Uuid;

pub type Games = HashMap<Uuid, Game>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Game {
    pub abort_reason: String,
    pub clocks: HashMap<ClockType, Clock>,
    pub clock_during_final_score: bool,
    pub current_period: Uuid,
    pub current_period_number: u8,
    pub current_timeout: String,
    pub event_info: EventInfo,
    pub export_blocked_by: String,
    pub filename: String,
    pub id: Uuid,
    pub in_jam: bool,
    pub in_overtime: bool,
    pub in_period: bool,
    pub in_sudden_scoring: bool,
    pub injury_continuation_upcoming: bool,
    pub json_exists: bool,
    pub labels: HashMap<LabelType, String>,
    pub name: String,
    pub name_format: String,
    pub no_more_jam: bool,
    pub official_review: bool,
    pub official_score: bool,
    pub penalty_codes: HashMap<PenaltyCode, String>,
    pub periods: Periods,
    pub rules: HashMap<String, String>,
    pub ruleset_name: String,
    pub state: String,
    pub statsbook_exists: bool,
    pub suspensions_served: String,
    pub teams: Teams,
    pub timeout_owner: String,
    pub upcoming_jam: Uuid,
    pub upcoming_jam_number: u8,
    pub update_in_progress: bool,
}

impl FromStateMap for Game {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        let mut result = Map::new();
        insert_base(state, &mut result, None);

        log::debug!("{:?}", state);

        // clocks
        let clocks = Clocks::from_state_map(&strip_prefix(state, "Clock."))?;
        result.insert("Clocks".to_string(), serde_json::to_value(clocks)?);

        let event_info = EventInfo::from_state_map(state)?;
        result.insert("EventInfo".to_string(), serde_json::to_value(event_info)?);

        // labels
        result.insert("Labels".to_string(), Value::Object(Map::new()));
        // penalty codes
        result.insert("PenaltyCodes".to_string(), Value::Object(Map::new()));

        // periods
        let period_state = strip_prefix(state, "Period.");
        let periods = Periods::from_state_map(&period_state)?;
        result.insert("Periods".to_string(), serde_json::to_value(periods)?);
        // rules
        result.insert("Rules".to_string(), Value::Object(Map::new()));
        // teams
        let team1 = Team::from_state_map(&strip_prefix(state, "Team.1."))?;
        let team2 = Team::from_state_map(&strip_prefix(state, "Team.2."))?;
        result.insert(
            "Teams".to_string(),
            Value::Array(vec![
                serde_json::to_value(team1)?,
                serde_json::to_value(team2)?,
            ]),
        );

        log::debug!("{:?}", result);

        Ok(serde_path_to_error::deserialize(result)?)
    }
}

impl FromStateMap for Games {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self> {
        let mut result = HashMap::new();

        let games_set = state
            .keys()
            .filter_map(|k| {
                if k.contains("ScoreBoard.Game") {
                    let uuid_str = k.split(".").nth(2)?;
                    Some(Uuid::from_str(uuid_str).unwrap())
                } else {
                    None
                }
            })
            .collect::<HashSet<Uuid>>();

        for game_id in games_set {
            let game_prefix = format!("ScoreBoard.Game.{}.", game_id);
            let game_state = state
                .iter()
                .filter_map(|(k, v)| {
                    if let Some(k) = k.strip_prefix(&game_prefix) {
                        Some((k.to_string(), v.clone()))
                    } else {
                        None
                    }
                })
                .collect::<Map<String, Value>>();

            let game = Game::from_state_map(&game_state)?;

            result.insert(game_id, game);
        }

        Ok(result)
    }
}
