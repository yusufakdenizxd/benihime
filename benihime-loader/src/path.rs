use anyhow::{Result, anyhow};
use std::env;
use std::path::PathBuf;

const APP_NAME: &str = "benihime";

fn home_dir() -> Result<PathBuf> {
    dirs_next::home_dir().ok_or_else(|| anyhow!("Home directory not found"))
}

fn xdg_dir(var: &str, fallback: &str) -> Result<PathBuf> {
    if let Ok(val) = env::var(var) {
        Ok(PathBuf::from(val))
    } else {
        Ok(home_dir()?.join(fallback))
    }
}

#[derive(Debug, Clone)]
pub struct Paths {
    pub config: PathBuf,
    pub data: PathBuf,
    pub cache: PathBuf,
}

impl Paths {
    pub fn new() -> Result<Self> {
        let config = xdg_dir("XDG_CONFIG_HOME", ".config")?.join(APP_NAME);
        let data = xdg_dir("XDG_DATA_HOME", ".local/share")?.join(APP_NAME);
        let cache = xdg_dir("XDG_CACHE_HOME", ".cache")?.join(APP_NAME);

        std::fs::create_dir_all(&config)?;
        std::fs::create_dir_all(&data)?;
        std::fs::create_dir_all(&cache)?;

        Ok(Self {
            config,
            data,
            cache,
        })
    }
}
