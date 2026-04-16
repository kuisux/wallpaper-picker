use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub wallpaper_dir: String,
    pub wallpaper_setter: String,
    pub matugen_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            wallpaper_dir: "~/Pictures/wallpapers".to_string(),
            wallpaper_setter: "awww".to_string(),
            matugen_enabled: true,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("wallpaper-picker")
            .join("config.toml");

        if !config_path.exists() {
            return Config::default();
        }

        let contents = std::fs::read_to_string(&config_path).unwrap_or_default();

        toml::from_str(&contents).unwrap_or_default()
    }

    pub fn resolved_wallpaper_dir(&self) -> PathBuf {
        if self.wallpaper_dir.starts_with("~/") {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(&self.wallpaper_dir[2..])
        } else {
            PathBuf::from(&self.wallpaper_dir)
        }
    }
}
