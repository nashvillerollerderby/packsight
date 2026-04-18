use crate::error::Result;
use crate::models::serde::GameFileState;
use std::env;

pub mod error;
pub mod models;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    log4rs::init_file("log4rs.yaml", Default::default()).expect("No log4rs.yaml file found");
    log::info!("{:?} {:?}", args[1], args[2]);

    let path = std::path::Path::new(&args[1]);
    let file = std::fs::read(path).ok().unwrap();

    let string = String::from_utf8(file.as_slice().to_vec()).ok().unwrap();
    let value = GameFileState::try_from(string)?;

    log::info!("{}", serde_json::to_string(&value).ok().unwrap());

    let path = std::path::Path::new(&args[2]);
    let file = std::fs::write(path, serde_json::to_string(&serde_json::to_value(value)?)?);
    Ok(())
}
