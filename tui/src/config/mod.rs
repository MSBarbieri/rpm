mod behavior;
mod key;
mod package;
mod theme;


pub use behavior::Behavior;
pub use key::KeyBindings;
pub use package::PackageManager;
pub use theme::Theme;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs::{create_dir, read_to_string, write};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub behavior: Behavior,
    pub theme: Theme,
    pub package_manager: PackageManager,
    pub key_bindings: KeyBindings,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            package_manager: PackageManager::default(),
            behavior: Behavior::default(),
            theme: Theme::default(),
            key_bindings: KeyBindings::default(),
        }
    }
}

const RPM_DIR: &str = "/etc/rpm";
impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub async fn load_config() -> Result<Config> {
        let mut config_path = PathBuf::from(RPM_DIR);
        config_path.push("rpm.conf");

        let home_config: Config = match config_path.as_path().exists() {
            true => {
                let home_string = read_to_string(config_path.as_path()).await.unwrap();
                toml::from_str::<Config>(&home_string)?
            }
            _ => Config::create_configs().await?,
        };

        Ok(home_config)
    }

    pub async fn create_configs() -> Result<Config> {
        let config = Config::new();
        config.save_configs().await?;
        Ok(config)
    }

    pub async fn save_configs(&self) -> Result<()> {
        let file = toml::to_string(self)?;
        let rpm_dir_path = PathBuf::from(RPM_DIR);
        if rpm_dir_path.exists() {
            let mut file_path = rpm_dir_path.clone();
            file_path.push("rpm.conf");
            if file_path.exists() {
                Ok(())
            } else {
                Ok(write(file_path.as_path(), file).await?)
            }
        } else {
            let mut file_path = rpm_dir_path.clone();
            create_dir(rpm_dir_path).await?;
            file_path.push("rpm.conf");
            Ok(write(file_path.as_path(), file).await?)
        }
    }
}
