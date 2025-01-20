const std = @import("std");
const term = @import("term.zig");
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
        const input = try reader.readByte();
        switch (input) {
            'q' => {
                try writer.print("Goodbye\n", .{});
                break;
            },
            else => {},
        }
    }
}
