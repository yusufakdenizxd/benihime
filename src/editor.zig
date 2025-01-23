const std = @import("std");
const io = std.io;
const fs = std.fs;
const keys = @import("keys.zig");
const buffer = @import("buffer.zig");
const t = @import("term.zig");

pub fn start(term: *t.RawTerm) !void {
    const allocator = std.heap.page_allocator;

    while (true) {
        try refreshScreen(term, allocator);
        const next = try keys.next(term.reader);
        switch (next) {
            .key => |k| switch (k) {
                .ctrl => |c| switch (c) {
                    'q' => break,
                    else => {},
                },
                .char => |c| switch (c) {
                    'h' => try moveCursor(c, term),
                    'j' => try moveCursor(c, term),
                    'k' => try moveCursor(c, term),
                    'l' => try moveCursor(c, term),
                    else => {},
                },
            },
            .none => try term.writer.print("Timeout.\n\r", .{}),

            else => {},
        }
    }
}

fn refreshScreen(term: *t.RawTerm, allocator: std.mem.Allocator) !void {
    var ab = buffer.Abuf.init(&allocator);
    defer ab.deinit();

    try ab.append("\x1b[?25l");

    try ab.append("\x1b[H");

    try drawRows(term, &ab);

    const cursorPosition = try std.fmt.allocPrint(allocator, "\x1b[{d};{d}H", .{ term.cy + 1, term.cx + 1 });
    try ab.append(cursorPosition);

    try ab.append("\x1b[?25h");

    _ = try term.writer.write(ab.buffer);
}

fn drawRows(term: *t.RawTerm, ab: *buffer.Abuf) !void {
    for (0..term.size.ws_row) |i| {
        if (i == term.size.ws_row / 3) {
            try drawWelcomeText(term, ab);
        } else {
            try ab.append("~");
        }
        try ab.append("\x1b[K");

        if (i < term.size.ws_row - 1) {
            try ab.append("\r\n");
        }
    }
}

fn drawWelcomeText(term: *t.RawTerm, ab: *buffer.Abuf) !void {
    const message = "Welcome to Benihime";

    var padding = (term.size.ws_col - message.len) / 2;
    if (padding != 0) {
        try ab.append("~");
        padding -= 1;
    }
    while (padding > 0) {
        try ab.append(" ");
        padding -= 1;
    }

    try ab.append(message);
}

fn moveCursor(char: u21, term: *t.RawTerm) !void {
    switch (char) {
        'h' => {
            term.cx -= 1;
        },
        'l' => {
            term.cx += 1;
        },
        'j' => {
            term.cy += 1;
        },
        'k' => {
            term.cy -= 1;
        },
        else => {},
    }
}
