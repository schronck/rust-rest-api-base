use config::{Config, File};
use std::{collections::HashMap, path::Path};
use thiserror::Error;

const CONFIG_PATH: &str = "config.json";

#[derive(Error, Debug)]
pub enum ConfigError {
	#[error(transparent)]
	Config(#[from] config::ConfigError),
	#[error("Value not found for key {0}")]
	NoSuchEntry(String),
}

fn read_config(key: &str) -> Result<String, ConfigError> {
	let config_path =
		std::env::var("CONFIG_PATH").unwrap_or(CONFIG_PATH.to_string());

	let settings = Config::builder()
		.add_source(File::from(Path::new(&config_path)))
		.build()?;

	let map = settings.try_deserialize::<HashMap<String, String>>()?;

	if let Some(value) = map.get(key).cloned() {
		Ok(value)
	} else {
		Err(ConfigError::NoSuchEntry(key.to_string()))
	}
}

macro_rules! create_config_fn {
	($func_name:ident) => {
		pub fn $func_name() -> String {
			match read_config(stringify!($func_name)) {
				Ok(value) => value,
				Err(e) => panic!("{e}"),
			}
		}
	};
	($func_name:ident, $default: expr) => {
		pub fn $func_name() -> String {
			match read_config(stringify!($func_name)) {
				Ok(value) => value,
				Err(_) => $default.into(),
			}
		}
	};
}

create_config_fn!(subscriber);
create_config_fn!(rust_log);
create_config_fn!(ip, "127.0.0.1");
create_config_fn!(port, "8081");
create_config_fn!(redis_url);
