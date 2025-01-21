const std = @import("std");
const term = @import("term.zig");
const keys = @import("keys.zig");
const io = std.io;

pub fn main() !void {
    const stdin = io.getStdIn();
    const stdout = io.getStdOut();

    const reader = stdin.reader();
    const writer = stdout.writer();

    try writer.print("Welcome to Benihime\n", .{});

    var raw_term = try term.enableRawMode(stdin.handle);
    defer raw_term.disableRawMode() catch {};

    while (true) {
        // const next = try events.nextWithTimeout(stdin, 1000);
        const next = try keys.next(reader);
        switch (next) {
            .key => |k| switch (k) {
                .ctrl => |c| switch (c) {
                    'q' => break,
                    else => try stdout.writer().print("ctrl+{u}\n\r", .{c}),
                },
                .char => |c| switch (c) {
                    else => try stdout.writer().print("{}\n\r", .{c}),
                },
            },
            .none => try stdout.writer().print("Timeout.\n\r", .{}),

            // ex. mouse events not supported yet
            else => try stdout.writer().print("Event: {}\n\r", .{next}),
        }
    }
}
