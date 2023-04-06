use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    paths: Paths,
    stedi: Stedi,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Paths {
    base: String,
    buyers: String,
    contracts: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Stedi {
    map_id: String,
    api_key: String,
    guide_id: String,
}

impl Config {
    pub fn new() -> Self {
        let config_toml_file = std::fs::read_to_string("config.toml");
        let config_toml = match config_toml_file {
            Ok(config_toml) => config_toml,
            Err(e) => panic!("Error reading config.toml: {}", e),
        };

        let config: Config = toml::from_str(config_toml.as_str()).unwrap();
        let base_path = config.paths.base.as_str();

        Self {
            paths: Paths {
                base: base_path.to_string(),
                buyers: format!("{}{}",base_path, config.paths.buyers.as_str()),
                contracts: format!("{}{}",base_path, config.paths.contracts.as_str()),
            },
            stedi: Stedi {
                map_id: config.stedi.map_id,
                api_key: config.stedi.api_key,
                guide_id: config.stedi.guide_id,
            },
        }
    }   

    pub fn get_buyers_path(&self, file_name: &str) -> PathBuf {
        let mut path_buf = PathBuf::new();
        path_buf.push(&self.paths.buyers.as_str());
        path_buf.push(format!("{}{}", file_name, ".csv"));
        
        path_buf
    }

    pub fn get_contracts_path(&self, file_name: &str) -> PathBuf {
        let mut path_buf = PathBuf::new();
        path_buf.push(self.paths.contracts.as_str());
        path_buf.push(format!("{}{}", file_name, ".csv").as_str());
        
        path_buf
    }

    pub fn get_stedi_params(&self) -> (&str, &str, &str) {
        (
            self.stedi.map_id.as_str(),
            self.stedi.api_key.as_str(),
            self.stedi.guide_id.as_str(),
        )
    }
}
