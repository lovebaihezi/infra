const std = @import("std");
const Allocator = std.mem.Allocator;
pub fn install(allocator: *Allocator) !void {
    const deps_root = try std.fs.realpathAlloc(allocator.*, "./third_party");
}
