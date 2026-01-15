use anyhow::{Context, Result};
use std::fs;

use crate::paths::Paths;

struct EmbeddedTheme {
    name: &'static str,
    contents: &'static str,
}

const DEFAULT_THEMES: &[EmbeddedTheme] = &[EmbeddedTheme {
    name: "default.toml",
    contents: include_str!("../../../runtime/themes/default.toml"),
}];

pub fn ensure_themes_exist(paths: &Paths) -> Result<()> {
    let themes_dir = paths.themes_dir();

    fs::create_dir_all(&themes_dir)
        .with_context(|| format!("Failed to create {:?}", themes_dir))?;

    for theme in DEFAULT_THEMES {
        let path = themes_dir.join(theme.name);

        if path.exists() {
            continue; // DO NOT overwrite user files
        }

        fs::write(&path, theme.contents).with_context(|| format!("Failed to write {:?}", path))?;
    }

    Ok(())
}
