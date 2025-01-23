const std = @import("std");
const term = @import("term.zig");
const keys = @import("keys.zig");
const editor = @import("editor.zig");
const io = std.io;

pub fn main() !void {
    const stdin = io.getStdIn();
    const stdout = io.getStdOut();

    const reader = stdin.reader();
    const writer = stdout.writer();

    var raw_term = try term.enableRawMode(stdin.handle, stdout.handle, reader, writer);
    defer raw_term.disableRawMode() catch {};

    // std.debug.print("Welcome to Benihime\n", .{});

    try editor.start(&raw_term);
}
