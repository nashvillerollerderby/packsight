pub mod clocks;
pub mod game;
pub mod game_file_state;
pub mod jam;
pub mod misc;
pub mod period;
pub mod team;
pub mod timeout;
mod utils;
pub mod version;

pub use clocks::*;
pub use game::*;
pub use game_file_state::*;
pub use jam::*;
pub use misc::*;
pub use period::*;
pub use team::*;
pub use timeout::*;
pub(crate) use utils::*;
pub use version::*;

use crate::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize)]
pub enum ScoreboardVersion {
    #[serde(rename = "v2025.5")]
    V2025_5,
    #[serde(rename = "v2025.8")]
    V2025_8,
}

pub type JsDate = DateTime<Utc>;

pub trait FromStateMap: Sized {
    fn from_state_map(state: &Map<String, Value>) -> Result<Self>;
}
