const std = @import("std");
const Allocator = std.mem.Allocator;

pub fn getNameWithExt(path: []const u8, split: u8) []const u8 {
    var len = path.len;
    while (len > 0) {
        len -= 1;
        if (path[len] == split) {
            return path[(len + 1)..];
        }
    }
    return path;
}

test "getNameWithExt" {
    const expect = std.testing.expect;
    const result1 = getNameWithExt("/sadf/asdf/asdf/asdf/grgs/wthh/sdgaer/asd", '/');
    try expect(std.mem.eql(u8, result1, "asd"));
    const result2 = getNameWithExt("bash", '/');
    try expect(std.mem.eql(u8, result2, "bash"));
}

/// return the absolute path of the name which could find in env PATH, or return null if not found
/// returned value will not auto be freed;
pub fn which(allocator: Allocator, name: []const u8) anyerror!?[]const u8 {
    // TODO: maybe env PATH does not exist on Windows
    if (std.os.getenv("PATH")) |paths| {
        var spliter = std.mem.split(u8, paths, ":");
        while (spliter.next()) |path| {
            var open_dir: anyerror!std.fs.Dir = std.fs.openDirAbsolute(path, std.fs.Dir.OpenDirOptions{ .iterate = true });
            if (open_dir) |opened_dir| {
                var dir = opened_dir;
                defer dir.close();
                var walk = try dir.walk(allocator);
                defer walk.deinit();
                while (try walk.next()) |entry| {
                    if (entry.kind == std.fs.Dir.Entry.Kind.File) {
                        const file_name = getNameWithExt(entry.path, std.fs.path.sep);
                        if (std.mem.eql(u8, file_name, name)) {
                            const all = [_][]const u8{ path, entry.path };
                            return try std.fs.path.join(allocator, &all);
                        }
                    }
                }
            } else |_| {
                std.log.warn("can not open {s} in PATH", .{path});
            }
        }
    }
    return null;
}

test "which bash" {
    const expect = std.testing.expect;
    var allocator = std.testing.allocator;
    const path = try which(allocator, "bash");
    try expect(path != null);
    if (path) |str| {
        defer allocator.free(str);
    }
}
