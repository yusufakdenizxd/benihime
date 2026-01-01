use crate::display_list::{DisplayList, DrawCommand, GlyphInstance};
use crate::frame_render::Frame;
use crate::graphics::{Color, FontId, Point, Rect};

pub struct Composer {
    pub font: FontId,
    pub char_width: u16,
    pub line_height: u16,
}

impl Composer {
    pub fn new(font: FontId, char_width: u16, line_height: u16) -> Self {
        Self {
            font,
            char_width,
            line_height,
        }
    }

    pub fn compose(&self, frame: &Frame) -> DisplayList {
        let mut list = DisplayList::new();

        list.commands
            .push(DrawCommand::Clear(Color::from_rgb(0, 0, 0)));

        for (row_idx, line) in frame.lines.iter().enumerate() {
            let y = (row_idx as u16) * self.line_height;
            if let Some(bg) = line.highlight {
                list.commands.push(DrawCommand::Rect {
                    rect: Rect::new(0, y, 1000, self.line_height),
                    color: bg,
                });
            }

            let mut glyphs = vec![];
            for (col_idx, _) in line.text.chars().enumerate() {
                glyphs.push(GlyphInstance {
                    glyph_index: 0,
                    position: Point {
                        x: (col_idx as u16) * self.char_width,
                        y,
                    },
                    color: Color::from_rgb(255, 255, 255),
                });
            }

            list.commands.push(DrawCommand::GlyphRun {
                font: self.font,
                glyphs,
            });
        }

        // TODO: cursors

        list
    }
}
