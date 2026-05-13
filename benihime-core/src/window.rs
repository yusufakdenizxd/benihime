use crate::{
    buffer::{BufferId, Position},
    editor::Mode,
    graphics::Rect,
};

slotmap::new_key_type! {
    pub struct WindowId;
}

#[derive(Debug)]
pub struct Window {
    pub id: WindowId,
    pub buffer_id: BufferId,
    pub cursor: Position,
    pub scroll_offset: usize,
    pub scroll_left: usize,
    pub rect: Rect,
    pub mode: Mode,
}

impl Window {
    pub fn new(buffer_id: BufferId) -> Self {
        Self {
            id: WindowId::default(),
            buffer_id,
            cursor: Position::start(),
            scroll_offset: 0,
            scroll_left: 0,
            rect: Rect::default(),
            mode: Mode::Normal,
        }
    }

    pub fn scroll_down(
        &mut self,
        lines: usize,
        screen_height: usize,
        scrolloff: usize,
        lines_len: usize,
    ) {
        let max_row = lines_len.saturating_sub(1);

        let new_row = (self.cursor.row + lines).min(max_row);
        self.cursor.row = new_row;

        if new_row >= self.scroll_offset.saturating_add(screen_height).saturating_sub(scrolloff) {
            self.scroll_offset = new_row.saturating_add(scrolloff + 1).saturating_sub(screen_height);
        }
        self.scroll_offset = self
            .scroll_offset
            .min(max_row.saturating_add(1).saturating_sub(screen_height));
    }

    pub fn scroll_up(&mut self, lines: usize, scrolloff: usize) {
        let new_row = self.cursor.row.saturating_sub(lines);
        self.cursor.row = new_row;

        if new_row < self.scroll_offset + scrolloff {
            self.scroll_offset = new_row.saturating_sub(scrolloff);
        }
    }

    pub fn center_cursor(&mut self, screen_height: usize, lines_len: usize) {
        let cursor_row = self.cursor.row.min(lines_len.saturating_sub(1));

        let half_screen = screen_height / 2;
        if cursor_row >= half_screen {
            self.scroll_offset = cursor_row.saturating_sub(half_screen);
        } else {
            self.scroll_offset = 0;
        }

        if self.scroll_offset + screen_height > lines_len {
            self.scroll_offset = lines_len.saturating_sub(screen_height);
        }
    }

    pub fn update_scroll(
        &mut self,
        screen_height: usize,
        vertical_scrolloff: usize,
        screen_width: usize,
        horizontal_scrolloff: usize,
    ) {
        let row = self.cursor.row;
        let col = self.cursor.col;

        if row < self.scroll_offset + vertical_scrolloff {
            self.scroll_offset = row.saturating_sub(vertical_scrolloff);
        }

        if row >= self.scroll_offset + screen_height - vertical_scrolloff {
            self.scroll_offset = row + vertical_scrolloff + 1 - screen_height;
        }

        if col < self.scroll_left + horizontal_scrolloff {
            self.scroll_left = col.saturating_sub(horizontal_scrolloff);
        } else if col >= self.scroll_left + screen_width - horizontal_scrolloff {
            self.scroll_left = col.saturating_sub(screen_width - horizontal_scrolloff);
        }
    }
}
