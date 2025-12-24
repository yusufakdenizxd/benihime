use super::color::Color;

pub struct BaseColors {
    pub bg0: Color,
    pub bg1: Color,
    pub bg2: Color,
    pub fg0: Color,
    pub fg1: Color,
    pub red: Color,
    pub green: Color,
    pub blue: Color,
    pub yellow: Color,
    pub orange: Color,
    pub purple: Color,
    pub cyan: Color,
    pub accent0: Color,
}

impl BaseColors {
    pub fn default() -> Self {
        Self {
            bg0: Color::from_rgb(29, 32, 33),
            bg1: Color::from_rgb(40, 40, 40),
            bg2: Color::from_rgb(60, 60, 60),
            fg0: Color::from_rgb(235, 219, 178),
            fg1: Color::from_rgb(255, 255, 255),
            red: Color::from_rgb(204, 36, 29),
            green: Color::from_rgb(152, 151, 26),
            blue: Color::from_rgb(69, 133, 136),
            yellow: Color::from_rgb(215, 153, 33),
            orange: Color::from_rgb(214, 93, 14),
            purple: Color::from_rgb(177, 98, 134),
            cyan: Color::from_rgb(42, 161, 152),
            accent0: Color::from_rgb(131, 139, 131),
        }
    }
}
