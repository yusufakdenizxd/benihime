use ropey::RopeSlice;

use crate::command::command::CommandContext;

use super::{movement, selection::Range};

pub fn move_word_impl<F>(cx: &mut CommandContext, move_fn: F)
where
    F: Fn(RopeSlice, Range, usize) -> Range,
{
    let state = &mut cx.state;
    let count = 1; // TODO: later read it from contexts repeat count

    let buf = state.focused_buf_mut();

    if buf.cursor.row < buf.lines.len_lines() {
        let rope_line: RopeSlice = buf.lines.line(buf.cursor.row);

        let selection = Range {
            anchor: buf.cursor.col,
            head: buf.cursor.col,
        };

        let new_range = move_fn(rope_line, selection, count);

        buf.cursor.col = new_range.head;
        buf.range = Some(new_range);
    }
}

pub fn move_next_word_start(cx: &mut CommandContext) {
    move_word_impl(cx, movement::move_next_word_start)
}

pub fn move_next_word_end(cx: &mut CommandContext) {
    move_word_impl(cx, movement::move_next_word_end)
}

pub fn move_prev_word_start(cx: &mut CommandContext) {
    move_word_impl(cx, movement::move_prev_word_start)
}

pub fn move_prev_word_end(cx: &mut CommandContext) {
    move_word_impl(cx, movement::move_prev_word_end)
}

pub fn move_next_long_word_start(cx: &mut CommandContext) {
    move_word_impl(cx, movement::move_next_long_word_start)
}

pub fn move_next_long_word_end(cx: &mut CommandContext) {
    move_word_impl(cx, movement::move_next_long_word_end)
}

pub fn move_prev_long_word_start(cx: &mut CommandContext) {
    move_word_impl(cx, movement::move_prev_long_word_start)
}

pub fn move_prev_long_word_end(cx: &mut CommandContext) {
    move_word_impl(cx, movement::move_prev_long_word_end)
}

pub fn move_next_sub_word_start(cx: &mut CommandContext) {
    move_word_impl(cx, movement::move_next_sub_word_start)
}

pub fn move_next_sub_word_end(cx: &mut CommandContext) {
    move_word_impl(cx, movement::move_next_sub_word_end)
}

pub fn move_prev_sub_word_start(cx: &mut CommandContext) {
    move_word_impl(cx, movement::move_prev_sub_word_start)
}

pub fn move_prev_sub_word_end(cx: &mut CommandContext) {
    move_word_impl(cx, movement::move_prev_sub_word_end)
}
