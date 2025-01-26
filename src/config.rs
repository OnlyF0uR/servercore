use jsonc_parser::parse_to_serde_value;
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};
use tokio::sync::OnceCell;

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigValue {
    pub db_path: String,
}

#[derive(Debug)]
pub struct Config {
    pub value: ConfigValue,
    path: String,
}

impl Config {
    pub async fn init(path: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Check if file exists tokio
        if Path::new(path).exists() {
            // Load file
            let data = tokio::fs::read_to_string(path).await?;
            let json_value = parse_to_serde_value(&data, &Default::default())?;

            if let Some(json_value) = json_value {
                let value: ConfigValue = serde_json::from_value(json_value)?;
                Ok(Config {
                    value,
                    path: path.to_owned(),
                })
            } else {
                Ok(Config::new(path).await)
            }
        } else {
            Ok(Config::new(path).await)
        }
    }

    pub async fn new(path: &str) -> Self {
        // Write a jsonc file with comments
        let contents = r#"
        {
            // Database path
            "db_path": "data.db"
        }"#;

        tokio::fs::write(path, contents).await.unwrap();

        let json_value = parse_to_serde_value(contents, &Default::default()).unwrap();
        let value: ConfigValue = serde_json::from_value(json_value.unwrap()).unwrap();

        Config {
            value,
            path: path.to_owned(),
        }

        // Or we could repeat the values we just entered
        // previously and return the Config struct
        // Config {
        //     value: ConfigValue {
        //         db_path: "data.db".to_string(),
        //     },
        //     path: path.to_owned(),
        // }
    }

    #[allow(dead_code)]
    pub async fn save(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let data = serde_json::to_string_pretty(&self.value)?;
        tokio::fs::write(&self.path, data).await?;

        Ok(())
    }
}

static CONFIG_INSTANCE: OnceCell<Arc<Config>> = OnceCell::const_new();

pub async fn setup_config(path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let path = format!("{}/config.jsonc", path);

    let db = Config::init(&path).await?;
    if let Err(e) = CONFIG_INSTANCE.set(Arc::new(db)) {
        return Err(e.into());
    };

    Ok(())
}

pub async fn get_config() -> Arc<Config> {
    CONFIG_INSTANCE.get().unwrap().clone()
}
