use anyhow::{Ok, Result, anyhow};
use std::path::PathBuf;
use toml::Value;

use super::theme::Theme;

pub struct ThemeLoader {
    dir: PathBuf,
}

impl ThemeLoader {
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    pub fn load(&self, name: &str) -> Result<Theme> {
        //TODO: Show warnings
        let (theme, warnings) = self.load_with_warnings(name)?;

        Ok(theme)
    }

    pub fn load_with_warnings(&self, name: &str) -> Result<(Theme, Vec<String>)> {
        let (theme, warnings) = self.load_theme(name).map(Theme::from_toml)?;

        let theme = Theme {
            name: name.into(),
            ..theme
        };

        Ok((theme, warnings))
    }

    fn load_theme(&self, name: &str) -> Result<Value> {
        let path = self.path(name)?;

        let theme_toml = self.load_toml(path)?;

        Ok(theme_toml)
    }

    fn load_toml(&self, path: PathBuf) -> Result<Value> {
        let data = std::fs::read_to_string(path)?;
        let value = toml::from_str(&data)?;

        Ok(value)
    }

    fn path(&self, name: &str) -> Result<PathBuf> {
        let filename = format!("{}.toml", name);
        let path = self.dir.with_file_name(filename);
        if path.is_file() {
            return Ok(path);
        }

        Err(anyhow!("File not found for: {}", name))
    }
}
