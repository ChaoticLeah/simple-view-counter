use serde::Deserialize;
use std::{error::Error, fs, io, path::{PathBuf}};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub allowed_origin: Option<String>,
    // pub port: u16,
    pub log_level: Option<String>,
    pub cooldown: Option<u64>,
    pub blacklist_ips: Option<Vec<String>>,
    pub allowed_keys: Option<Vec<String>>,
}

pub async fn load_config(path: PathBuf) -> Result<Config, Box<dyn Error>> {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let file_path = exe_dir.join(path);

    let config_file_string = fs::read_to_string(file_path).map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Cannot parse config.yaml. Place it in the same directory as the .exe",
        )
    })?;
    let config: Config = serde_yaml::from_str(&config_file_string)?;
    Ok(config)
}