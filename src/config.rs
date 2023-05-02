use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

const CONFIG_FILE: &str = "config.toml";
const APP_CONFIG_DIR: &str = "osintui";

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub keys: Keys,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Keys {
    pub virustotal: String,
    pub shodan: String,
    pub censys_id: String,
    pub censys_secret: String,
}

pub struct ConfigPaths {
    pub config_file_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Config {
        Config {
            keys: Keys {
                virustotal: "".to_string(),
                shodan: "".to_string(),
                censys_id: "".to_string(),
                censys_secret: "".to_string(),
            },
        }
    }

    pub fn get_or_build_paths(&self) -> Result<ConfigPaths> {
        match dirs::config_dir() {
            Some(config_dir) => {
                let app_config_dir = config_dir.join(APP_CONFIG_DIR);

                if !app_config_dir.exists() {
                    fs::create_dir_all(&app_config_dir)?;
                }

                let config_file_path = app_config_dir.join(CONFIG_FILE);

                let paths = ConfigPaths { config_file_path };

                Ok(paths)
            }
            None => Err(anyhow!(
                "No $XDG_CONFIG_HOME or $HOME directory found for client config"
            )),
        }
    }

    pub fn load_config(&mut self) -> Result<()> {
        let paths = self.get_or_build_paths()?;
        if paths.config_file_path.exists() {
            let config_file = fs::read_to_string(&paths.config_file_path)?;
            let config_toml: Config = toml::from_str(&config_file)?;

            self.keys.shodan = config_toml.keys.shodan;
            self.keys.virustotal = config_toml.keys.virustotal;
            self.keys.censys_id = config_toml.keys.censys_id;
            self.keys.censys_secret = config_toml.keys.censys_secret;

            Ok(())
        } else {
            println!(
                "Config will be saved to {}",
                paths.config_file_path.display()
            );
            fs::File::create(&paths.config_file_path)?;
            let config = Config::default();
            let contents = toml::to_string(&config).expect("Could not encode TOML value");
            fs::write(paths.config_file_path, contents).expect("Could not write to file!");
            Ok(())
        }
    }
}
