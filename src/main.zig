const std = @import("std");
const io = std.io;

pub fn main() !void {
    const reader = io.getStdIn().reader();
    const writer = io.getStdOut().writer();

    try writer.print("Welcome to Benihime\n", .{});

    while (true) {
        const input = try reader.readByte();
        switch (input) {
            else => {},
        }
    }
}
