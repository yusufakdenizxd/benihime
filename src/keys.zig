const std = @import("std");
const io = std.io;

pub const Event = union(enum) {
    key: Key,
    resize,
    not_supported,
    none, // polling timeout
};

const Key = union(enum) {
    char: u21,
    ctrl: u21,
};

pub fn next(in: anytype) !Event {
    var buf: [20]u8 = undefined;
    const len = try in.read(&buf);
    if (len == 0) {
        return .none;
    }

    const view = try std.unicode.Utf8View.init(buf[0..len]);

    var iter = view.iterator();
    const event: Event = .none;

    // TODO: Find a better way to iterate buffer
    if (iter.nextCodepoint()) |c0| switch (c0) {
        '\x01'...'\x0C', '\x0E'...'\x1A' => return Event{ .key = Key{ .ctrl = c0 + '\x60' } },

        else => return Event{ .key = Key{ .char = c0 } },
    };

    return event;
}
