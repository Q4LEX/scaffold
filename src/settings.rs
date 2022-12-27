use std::{
    fs::{self, File},
    io::Write,
};

use serde::{Deserialize, Serialize};
use tracing::{error, trace, warn};

const SETTINGS_FILE_PATH: &str = "settings.yaml";

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
#[serde(rename_all = "lowercase")]
pub struct Settings {
    fps_pref: FpsPref,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            fps_pref: FpsPref::VSync,
        }
    }
}

impl Settings {
    // Tries reading config file, if it does not exist, return the default.
    pub fn read_from_file() -> Self {
        match fs::read_to_string(SETTINGS_FILE_PATH) {
            Ok(content) => match serde_yaml::from_str(&content) {
                Ok(settings) => {
                    trace!("Successfully parsed settings from file");
                    settings
                }
                Err(err) => {
                    warn!("Unable to deserialize settings from file: {}", err);
                    trace!("Falling back to default settings");
                    Settings::default()
                }
            },
            Err(err) => {
                warn!("Unable to open settings from file: {}", err);
                trace!("Falling back to default settings");
                Settings::default()
            }
        }
    }

    pub fn save_to_file(&self) {
        let serialized = match serde_yaml::to_string(&self) {
            Ok(serialized) => serialized,
            Err(err) => {
                error!("Unable to serialize settings: {}", err);
                return;
            }
        };
        let mut file = match fs::metadata(SETTINGS_FILE_PATH) {
            Ok(_) => match File::create(SETTINGS_FILE_PATH) {
                Ok(file) => file,
                Err(err) => {
                    error!("Unable to open settings file: {}", err);
                    return;
                }
            },
            Err(_) => {
                trace!("Settings file does not exist, creating file");
                match File::create(SETTINGS_FILE_PATH) {
                    Ok(file) => file,
                    Err(err) => {
                        error!("Unable to create settings file: {}", err);
                        return;
                    }
                }
            }
        };

        match file.write_all(serialized.as_bytes()) {
            Ok(_) => trace!("Settings file written"),
            Err(err) => error!("Unable to write bytes into settings file: {}", err),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FpsPref {
    VSync,
    // With max fps
    Immediate(u32),
}
