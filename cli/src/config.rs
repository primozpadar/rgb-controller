use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::io::ErrorKind;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub address: String,

  #[serde(skip_serializing, skip_deserializing)]
  path: String,
}

type ConfigResult = Result<Config, Box<dyn Error>>;

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

  fn save(&self) -> Result<(), Box<dyn Error>> {
    let file = File::create(self.path.as_str())?;
    serde_json::to_writer_pretty(file, &self)?;
    Ok(())
  }

  pub fn update_address(&mut self, new_address: &str) -> Result<(), Box<dyn Error>> {
    let socket_addr: SocketAddr = new_address.parse().expect("Invalid new address");
    self.address = socket_addr.to_string();
    self.save()
  }

  pub fn get_default() -> Config {
    Config {
      address: String::from("192.168.1.255:50000"),
      path: String::from(""),
    }
  }
}

fn create_new(path: &str) -> ConfigResult {
  let defualt_config = Config::get_default();
  let file = File::create(path)?;
  serde_json::to_writer_pretty(file, &defualt_config)?;
  return Ok(defualt_config);
}
