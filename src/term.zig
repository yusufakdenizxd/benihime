const std = @import("std");

const os = std.os;
const fs = std.fs;
const posix = std.posix;

const TIOCGWINSZ = 0x5413; // ioctl code for getting window size on Linux

pub const RawTerm = struct {
    orig_termios: std.posix.termios,
    size: std.posix.system.winsize,

    inHandle: os.linux.fd_t,
    outHandle: os.linux.fd_t,

    reader: fs.File.Reader,
    writer: fs.File.Writer,

    const Self = @This();

    pub fn disableRawMode(self: *Self) !void {
        try posix.tcsetattr(self.inHandle, .FLUSH, self.orig_termios);
    }
};

pub fn enableRawMode(inHandle: posix.fd_t, outHandle: posix.fd_t, reader: fs.File.Reader, writer: fs.File.Writer) !RawTerm {
    const original_termios = try posix.tcgetattr(inHandle);

    var termios = original_termios;

    // https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html
    // TCSETATTR(3)
    // reference: void cfmakeraw(struct termios *t)

    termios.iflag.BRKINT = false;
    termios.iflag.ICRNL = false;
    termios.iflag.INPCK = false;
    termios.iflag.ISTRIP = false;
    termios.iflag.IXON = false;

    termios.oflag.OPOST = false;

    termios.lflag.ECHO = false;
    termios.lflag.ICANON = false;
    termios.lflag.IEXTEN = false;
    termios.lflag.ISIG = false;

    termios.cflag.CSIZE = .CS8;

    termios.cc[@intFromEnum(posix.V.MIN)] = 1;
    termios.cc[@intFromEnum(posix.V.TIME)] = 0;

    try posix.tcsetattr(inHandle, .FLUSH, termios);

    var ws: std.posix.system.winsize = undefined;
    _ = std.os.linux.ioctl(outHandle, TIOCGWINSZ, @intFromPtr(&ws));

    return RawTerm{
        .orig_termios = original_termios,
        .inHandle = inHandle,
        .outHandle = outHandle,
        .reader = reader,
        .writer = writer,
        .size = ws,
    };
}
