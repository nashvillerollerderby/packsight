use crate::models::serde::{FromStateMap, Game, Games, Version};
use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Serialize)]
pub struct GameFileState {
    games: HashMap<uuid::Uuid, Game>,
    version: Version,
}

impl TryFrom<String> for GameFileState {
    type Error = crate::error::PacksightError;

    fn try_from(value: String) -> crate::error::Result<Self> {
        let map_from_string: Map<String, Value> = Map::from_str(&value)?;
        let state = map_from_string
            .get("state")
            .unwrap()
            .as_object()
            .unwrap()
            .clone();
        let mut game_file_state = Map::new();
        for (k, v) in state.into_iter() {
            let no_left_paren = k.replacen('(', ".", usize::MAX);
            let no_right_paren = no_left_paren.replacen(')', "", usize::MAX);
            log::trace!("{}", no_right_paren);
            game_file_state.insert(no_right_paren, v.clone());
        }

        let version = Version::from_state_map(&game_file_state)?;

        let games = Games::from_state_map(&game_file_state)?;

        Ok(GameFileState { games, version })
    }
}
