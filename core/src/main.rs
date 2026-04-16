use serde::Deserialize;

use crate::game::GameFileState;
use crate::error::Result;

pub mod game;
pub mod error;
pub mod models;

fn main() -> Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).expect("No log4rs.yaml file found");

    let path = std::path::Path::new("../example-data/crg-game-2025-03-24__Penguins_vs._Polar_Bears_(Finished__129_-_87).json");
    let file = std::fs::read(path).ok().unwrap();

    let string = String::from_utf8(file.as_slice().to_vec()).ok().unwrap();
    let value = GameFileState::try_from(string)?;

    // let wah = serde_json::from_slice::<game::GameFileState>(file.as_slice()).ok().unwrap();
    log::info!("{}", serde_json::to_string(&value).ok().unwrap());
    Ok(())
}
