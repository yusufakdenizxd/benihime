const std = @import("std");
const io = std.io;
const fs = std.fs;
const keys = @import("keys.zig");
const t = @import("term.zig");

pub fn start(term: t.RawTerm) !void {
    try clearScreen(term.writer);
    for (0..24) |_| {
        try term.writer.print("~\r\n", .{});
    }

    while (true) {
        const next = try keys.next(term.reader);
        switch (next) {
            .key => |k| switch (k) {
                .ctrl => |c| switch (c) {
                    'q' => break,
                    else => try term.writer.print("ctrl+{u}\n\r", .{c}),
                },
                .char => |c| switch (c) {
                    else => try term.writer.print("{}\n\r", .{c}),
                },
            },
            .none => try term.writer.print("Timeout.\n\r", .{}),

            else => try term.writer.print("Event: {}\n\r", .{next}),
        }
    }
}

fn clearScreen(writer: fs.File.Writer) !void {
    try writer.print("\x1b[2J", .{});
    try writer.print("\x1b[H", .{});
}
