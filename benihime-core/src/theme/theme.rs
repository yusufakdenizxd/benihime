use std::collections::HashMap;

use benihime_renderer::graphics::{Color, Style, Modifier};
use toml::{Value, map::Map};

use crate::hashmap;

pub struct ColorPalette {
    palette: HashMap<String, Color>,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            palette: hashmap! {
                "default".to_string() => Color::default(),
                "black".to_string() => Color::from_hex_string("#282828"),
                "red".to_string() => Color::from_hex_string("#fb4934"),
                "green".to_string() => Color::from_hex_string("##62693e"),
                "yellow".to_string() => Color::from_hex_string("#d79921"),
                "blue".to_string() => Color::from_hex_string("#458588"),
                "magenta".to_string() => Color::from_hex_string("#b16286"),
                "cyan".to_string() => Color::from_hex_string("#689d6a"),
                "gray".to_string() => Color::from_hex_string("#928374"),
                "light-red".to_string() => Color::from_hex_string("#fc9487"),
                "light-green".to_string() => Color::from_hex_string("#d5d39b"),
                "light-yellow".to_string() => Color::from_hex_string("#fabd2f"),
                "light-blue".to_string() => Color::from_hex_string("#83a598"),
                "light-magenta".to_string() => Color::from_hex_string("#d3869b"),
                "light-cyan".to_string() => Color::from_hex_string("#8ec07c"),
                "light-gray".to_string() => Color::from_hex_string("#3c3836"),
                "white".to_string() => Color::from_hex_string("#f9f5d7"),
            },
        }
    }
}

impl TryFrom<Value> for ColorPalette {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let map = match value {
            Value::Table(entries) => entries,
            _ => return Ok(Self::default()),
        };

        let mut palette = HashMap::with_capacity(map.len());
        for (name, value) in map {
            let value = Self::parse_value_as_str(&value)?;
            let color = Color::from_hex_string(value);
            palette.insert(name, color);
        }

        Ok(Self::new(palette))
    }
}

impl ColorPalette {
    pub fn new(palette: HashMap<String, Color>) -> Self {
        let mut color_palette = ColorPalette::default();

        color_palette.palette.extend(palette);

        color_palette
    }

    fn parse_value_as_str(value: &Value) -> Result<&str, String> {
        value
            .as_str()
            .ok_or(format!("Unrecognized value: {}", value))
    }

    pub fn parse_color(&self, value: Value) -> Result<Color, String> {
        let value = Self::parse_value_as_str(&value)?;

        self.palette
            .get(value)
            .copied()
            .ok_or(format!("Invalid Color: {}", value))
    }

    pub fn parse_modifier(value: &Value) -> Result<Modifier, String> {
        value
            .as_str()
            .and_then(|s| s.parse().ok())
            .ok_or(format!("Invalid modifier: {}", value))
    }

    pub fn parse_style(&self, style: &mut Style, value: Value) -> Result<(), String> {
        if let Value::Table(entries) = value {
            for (name, value) in entries {
                match name.as_str() {
                    "fg" => *style = style.fg(self.parse_color(value)?),
                    "bg" => *style = style.bg(self.parse_color(value)?),
                    "modifiers" => {
                        let modifiers = value.as_array().ok_or("Modifiers should be an array")?;

                        for modifier in modifiers {
                            *style = style.add_modifier(Self::parse_modifier(modifier)?);
                        }
                    }
                    _ => return Err(format!("Invalid style attribute: {}", name)),
                }
            }
        } else {
            *style = style.fg(self.parse_color(value)?);
        }
        Ok(())
    }
}

pub struct Theme {
    pub name: String,
    pub colors: ColorPalette,
    pub styles: HashMap<String, Style>,
}

impl From<Value> for Theme {
    fn from(value: Value) -> Self {
        //TODO: Show warnings
        let (theme, warnings) = Theme::from_toml(value);
        theme
    }
}
impl Default for Theme {
    fn default() -> Self {
        Theme {
            name: "Default".into(),
            colors: ColorPalette::default(),
            styles: HashMap::new(),
        }
    }
}

fn build_theme_values(
    mut values: Map<String, Value>,
) -> (HashMap<String, Style>, ColorPalette, Vec<String>) {
    let mut styles = HashMap::new();
    let mut warnings = Vec::new();

    let palette = values
        .remove("palette")
        .map(|value| {
            ColorPalette::try_from(value).unwrap_or_else(|err| {
                warnings.push(err);
                ColorPalette::default()
            })
        })
        .unwrap_or_default();

    styles.reserve(values.len());

    for (name, style_value) in values {
        let mut style = Style::default();
        if let Err(err) = palette.parse_style(&mut style, style_value) {
            warnings.push(format!("Failed to parse style for key {name:?}. {err}"));
        }

        styles.insert(name.clone(), style);
    }

    (styles, palette, warnings)
}

impl Theme {
    pub fn get(&self, scope: &str) -> Style {
        self.try_get(scope).unwrap_or_default()
    }

    pub fn try_get(&self, scope: &str) -> Option<Style> {
        std::iter::successors(Some(scope), |s| Some(s.rsplit_once('.')?.0))
            .find_map(|s| self.styles.get(s).copied())
    }

    pub fn from_toml(value: Value) -> (Self, Vec<String>) {
        if let Value::Table(table) = value {
            Theme::from_keys(table)
        } else {
            Default::default()
        }
    }

    fn from_keys(toml_keys: Map<String, Value>) -> (Self, Vec<String>) {
        let (styles, palette, load_errors) = build_theme_values(toml_keys);

        let theme = Self {
            colors: palette,
            styles,
            ..Default::default()
        };
        (theme, load_errors)
    }
}
