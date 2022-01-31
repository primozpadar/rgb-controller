use crate::utils::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub address: String,
    pub default_id: String,
    pub presets: HashMap<String, Color>,

    #[serde(skip_serializing, skip_deserializing)]
    path: String,
}

type ConfigResult = Result<Config, Box<dyn Error>>;
type ConfigUpdateResult = Result<(), Box<dyn Error>>;

impl Config {
    // find file or create new if it doesnt exist
    pub fn load_or_create(path: &str) -> ConfigResult {
        let raw_config_result = fs::read_to_string(path);
        let mut config = match raw_config_result {
            Ok(raw_config) => serde_json::from_str::<Config>(&raw_config)?,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => create_new(path)?,
                _ => return Err(Box::new(error)),
            },
        };

        config.path = path.to_string();
        return Ok(config);
    }

    fn save(&self) -> ConfigUpdateResult {
        let file = File::create(self.path.as_str())?;
        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }

    pub fn update_address(&mut self, new_address: &str) -> ConfigUpdateResult {
        let socket_addr: SocketAddr = new_address.parse().expect("Invalid new address");
        self.address = socket_addr.to_string();
        self.save()
    }

    pub fn update_default_id(&mut self, new_default_id: &str) -> ConfigUpdateResult {
        self.default_id = new_default_id.to_string();
        self.save()
    }

    pub fn add_preset(&mut self, name: &str, color: Color) -> ConfigUpdateResult {
        self.presets.insert(name.to_string(), color);
        self.save()
    }

    pub fn remove_preset(&mut self, name: &str) -> ConfigUpdateResult {
        self.presets.remove(name);
        self.save()
    }

    pub fn get_default() -> Config {
        Config {
            address: String::from("192.168.1.255:50000"),
            default_id: String::from("00"),
            path: String::from(""),
            presets: get_default_presets(),
        }
    }
}

fn create_new(path: &str) -> ConfigResult {
    let defualt_config = Config::get_default();
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, &defualt_config)?;
    return Ok(defualt_config);
}

fn get_default_presets() -> HashMap<String, Color> {
    let mut presets: HashMap<String, Color> = HashMap::new();
    presets.insert("red".to_string(), Color { r: 255, g: 0, b: 0 });
    presets.insert("green".to_string(), Color { r: 0, g: 255, b: 0 });
    presets.insert("blue".to_string(), Color { r: 0, g: 0, b: 255 });
    presets.insert("black".to_string(), Color { r: 0, g: 0, b: 0 });
    presets.insert("white".to_string(), Color { r: 255, g: 255, b: 255 });
    presets
}
