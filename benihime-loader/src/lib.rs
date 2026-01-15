pub mod paths;
pub mod themes;

use anyhow::Result;

use crate::paths::Paths;

pub struct Loader {
    pub paths: Paths,
}

impl Loader {
    pub fn new() -> Result<Self> {
        let paths = Paths::new()?;
        themes::bootstrap::ensure_themes_exist(&paths)?;
        Ok(Self { paths })
    }
}
