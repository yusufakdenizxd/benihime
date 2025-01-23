const std = @import("std");

pub const Abuf = struct {
    buffer: []u8,
    allocator: *const std.mem.Allocator,

    pub fn init(allocator: *const std.mem.Allocator) Abuf {
        return Abuf{
            .buffer = &[_]u8{},
            .allocator = allocator,
        };
    }

    pub fn append(self: *Abuf, data: []const u8) !void {
        const new_len = self.buffer.len + data.len;

        var new_buffer = try self.allocator.realloc(self.buffer, new_len);

        @memcpy(new_buffer[self.buffer.len..], data);

        self.buffer = new_buffer;
    }

    pub fn deinit(self: *Abuf) void {
        self.allocator.free(self.buffer);
        self.buffer = &[_]u8{};
    }
};
