use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;

const CFG_FNAME: &str = "./config.toml";

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Config {
    min_silence_duration: f32,
    min_speech_duration: f32,
    max_speech_duration: f32,
}

impl Config {
    pub fn write_file(&self) -> Result<()> {
        let mut file = std::fs::File::create(CFG_FNAME)?;
        let v = toml::to_string(&self)?;
        file.write_all(v.as_bytes())?;

        Ok(())
    }

    pub fn read_file() -> Result<Config> {
        let contents = std::fs::read_to_string(CFG_FNAME)?;
        let cfg: Config = toml::from_str(&contents)?;

        spdlog::info!("Config: {cfg:?}");

        Ok(cfg)
    }
}

// async fn get_config() -> Json<Config> {
//     let cfg = Config::read_file().unwrap_or(Config::default());
//     Json(cfg)
// }

// async fn set_config(extract::Json(mut cfg): extract::Json<Config>) -> Json<String> {
//     if let Err(e) = cfg.write_file() {
//         spdlog::error!("{e}");
//     }

//     Json("ok".to_string())
// }
