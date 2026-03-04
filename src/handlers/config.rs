use std::env;
use std::env::var;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub lang: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lang: system_lang()
        }
    }
}

pub fn config_read() -> Result<Config, String> {
    let path = format!("{}/.config/en-os/remote_assistant/settings.json",var("HOME").unwrap_or("/tmp/".to_string()));
    let path2 = format!("{}/.config/en-os/remote_assistant/",var("HOME").unwrap_or("/tmp/".to_string()));
    let path = Path::new(&path);
    let path2 = Path::new(&path2);

    match path.exists() {
        true => {
            log::info!("Config exists, read file");
            let file = File::open(path).map_err(|e| e.to_string())?;
            let reader = BufReader::new(file);
            let config: Config = serde_json::from_reader(reader)
            .map_err(|e| format!("Error parsing config: {}", e))?;
            Ok(config)
        }

        false => {
            log::warn!("Read error, maybe config has not been created?");
            fs::create_dir_all(path2).map_err(|e| e.to_string())?;

            let default_config = Config::default();
            let file = fs::File::create(path).map_err(|e| e.to_string())?;
            let mut writer = BufWriter::new(file);

            serde_json::to_writer_pretty(&mut writer, &default_config).map_err(|e| e.to_string())?;
            writer.flush().map_err(|e| e.to_string())?;

            Ok(default_config)
        }


    }
}

pub fn config_write(config: Config) -> Result<Config, String> {
    log::info!("Write to config...");

    let path = format!("{}/.config/en-os/remote_assistant/settings.json",var("HOME").unwrap_or("/tmp/".to_string()));
    let path2 = format!("{}/.config/en-os/remote_assistant/",var("HOME").unwrap_or("/tmp/".to_string()));
    let path = Path::new(&path);
    let path2 = Path::new(&path2);

    fs::create_dir_all(path2).map_err(|e| format!("Ошибка создания директории для конфига: {}", e))?;

    let file = File::create(path)
    .map_err(|e| format!("Ошибка создания конфига: {}", e))?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, &config)
    .map_err(|e| format!("Ошибка записи конфига: {}", e))?;

    Ok(config)
}

fn system_lang() -> String {
    env::var("LANG").expect("Ошибка получения языка").to_string().chars().take(2).collect()
}
