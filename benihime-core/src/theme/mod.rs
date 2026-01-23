use std::{collections::HashMap, str::FromStr};

use toml::{Value, map::Map};

use bitflags::bitflags;

use crate::hashmap;

use egui::{
    style::{WidgetVisuals, Widgets},
    Color32, Stroke, Visuals,
};

pub mod theme_loader;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hex_string(s: &str) -> Self {
        if s.len() >= 7 {
            if let (Ok(red), Ok(green), Ok(blue)) = (
                u8::from_str_radix(&s[1..3], 16),
                u8::from_str_radix(&s[3..5], 16),
                u8::from_str_radix(&s[5..7], 16),
            ) {
                return Color::from_rgb(red, green, blue);
            }
        }
        Color::default()
    }
}

impl From<Color> for egui::Color32 {
    fn from(c: Color) -> Self {
        egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
    }
}

bitflags! {
    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    pub struct Modifier: u16 {
        const BOLD              = 0b0000_0000_0001;
        const ITALIC            = 0b0000_0000_0010;
        const CROSSED_OUT       = 0b0000_0000_0100;
    }
}

impl FromStr for Modifier {
    type Err = &'static str;

    fn from_str(modifier: &str) -> Result<Self, Self::Err> {
        match modifier {
            "bold" => Ok(Self::BOLD),
            "italic" => Ok(Self::ITALIC),
            "crossed_out" => Ok(Self::CROSSED_OUT),
            _ => Err("Invalid modifier"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HighlightGroup {
    pub fg: Option<Color>,
    pub bg: Option<Color>,

    pub modifier: Modifier,
}

impl Default for HighlightGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl HighlightGroup {
    pub const fn new() -> Self {
        HighlightGroup {
            fg: None,
            bg: None,
            modifier: Modifier::empty(),
        }
    }
    pub const fn fg(mut self, color: Color) -> HighlightGroup {
        self.fg = Some(color);
        self
    }

    pub const fn bg(mut self, color: Color) -> HighlightGroup {
        self.bg = Some(color);
        self
    }

    pub fn add_modifier(mut self, modifier: Modifier) -> HighlightGroup {
        self.modifier.insert(modifier);
        self
    }

    pub fn remove_modifier(mut self, modifier: Modifier) -> HighlightGroup {
        self.modifier.remove(modifier);
        self
    }

    pub fn patch(mut self, other: HighlightGroup) -> HighlightGroup {
        self.fg = other.fg.or(self.fg);
        self.bg = other.bg.or(self.bg);

        self.modifier.insert(other.modifier);

        self
    }
}

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

    pub fn parse_style(&self, style: &mut HighlightGroup, value: Value) -> Result<(), String> {
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
    pub groups: HashMap<String, HighlightGroup>,
}

impl From<Value> for Theme {
    fn from(value: Value) -> Self {
        //TODO: Show warnings
        let (theme, _warnings) = Theme::from_toml(value);
        theme
    }
}
impl Default for Theme {
    fn default() -> Self {
        Theme {
            name: "Default".into(),
            colors: ColorPalette::default(),
            groups: HashMap::new(),
        }
    }
}

fn build_theme_values(
    mut values: Map<String, Value>,
) -> (HashMap<String, HighlightGroup>, ColorPalette, Vec<String>) {
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
        let mut style = HighlightGroup::default();
        if let Err(err) = palette.parse_style(&mut style, style_value) {
            warnings.push(format!("Failed to parse style for key {name:?}. {err}"));
        }

        styles.insert(name.clone(), style);
    }

    (styles, palette, warnings)
}

impl Theme {
    pub fn get(&self, scope: &str) -> HighlightGroup {
        self.try_get(scope).unwrap_or_default()
    }

    pub fn try_get(&self, scope: &str) -> Option<HighlightGroup> {
        std::iter::successors(Some(scope), |s| Some(s.rsplit_once('.')?.0))
            .find_map(|s| self.groups.get(s).copied())
    }

    pub fn from_toml(value: Value) -> (Self, Vec<String>) {
        if let Value::Table(table) = value {
            Theme::from_keys(table)
        } else {
            Default::default()
        }
    }

    fn from_keys(toml_keys: Map<String, Value>) -> (Self, Vec<String>) {
        let (groups, palette, load_errors) = build_theme_values(toml_keys);

        let theme = Self {
            colors: palette,
            groups,
            ..Default::default()
        };
        (theme, load_errors)
    }

    pub fn to_egui_visuals(&self) -> Visuals {
        let mut visuals = Visuals::dark();

        if let Some(bg) = self.get("ui.background").bg {
            visuals.panel_fill = bg.into();
            visuals.window_fill = bg.into();
        }

        let text_color = self
            .get("ui.text")
            .fg
            .map(Into::into)
            .unwrap_or(visuals.override_text_color.unwrap_or(Color32::WHITE));
        visuals.override_text_color = Some(text_color);

        if let Some(bg) = self.get("ui.background").bg {
            visuals.faint_bg_color = bg.into();
        }
        if let Some(bg) = self.get("ui.cursorline").bg {
            visuals.extreme_bg_color = bg.into();
        }
        if let Some(bg) = self.get("ui.selection").bg {
            visuals.code_bg_color = bg.into();
        }

        if let Some(fg) = self.get("warning").fg {
            visuals.warn_fg_color = fg.into();
        }
        if let Some(fg) = self.get("error").fg {
            visuals.error_fg_color = fg.into();
        }

        let noninteractive_group = self.get("ui.menu");
        let noninteractive = WidgetVisuals {
            weak_bg_fill: noninteractive_group
                .bg
                .map(Into::into)
                .unwrap_or(visuals.panel_fill),
            bg_fill: noninteractive_group
                .bg
                .map(Into::into)
                .unwrap_or(visuals.panel_fill),
            bg_stroke: Stroke::NONE,
            fg_stroke: Stroke::new(
                1.0,
                noninteractive_group
                    .fg
                    .map(Into::into)
                    .unwrap_or(text_color),
            ),
            corner_radius: 0.0.into(),
            expansion: 0.0,
        };

        let menu_selected = self.get("ui.menu.selected");
        let hovered = WidgetVisuals {
            bg_fill: menu_selected
                .bg
                .map(Into::into)
                .unwrap_or(noninteractive.bg_fill),
            bg_stroke: Stroke::NONE, // TODO: Borders
            fg_stroke: Stroke::new(
                1.0,
                menu_selected.fg.map(Into::into).unwrap_or(text_color),
            ),
            ..noninteractive
        };

        let text_focus = self.get("ui.text.focus");
        let active = WidgetVisuals {
            bg_fill: text_focus.bg.map(Into::into).unwrap_or(hovered.bg_fill),
            bg_stroke: Stroke::NONE, // TODO: Borders
            fg_stroke: Stroke::new(
                1.0,
                text_focus.fg.map(Into::into).unwrap_or(text_color),
            ),
            ..hovered
        };

        visuals.widgets = Widgets {
            noninteractive,
            inactive: noninteractive,
            hovered,
            active,
            open: noninteractive,
        };

        if let Some(bg) = self.get("ui.selection").bg {
            visuals.selection.bg_fill = bg.into();
        }

        visuals.selection.stroke = Stroke::NONE;

        visuals
    }
}
