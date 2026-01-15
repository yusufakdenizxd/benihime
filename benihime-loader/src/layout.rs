use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RuntimeLayout {
    pub themes: PathBuf,
    pub config: PathBuf,
}

impl RuntimeLayout {
    pub fn from_xdg(config_dir: PathBuf) -> Self {
        let themes = config_dir.join("themes");
        let config = config_dir.join("config.toml");

        std::fs::create_dir_all(&themes).ok();

        Self { themes, config }
    }
}
