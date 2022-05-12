const std = @import("std");
const proto_compile = @import("proto/compile.zig");
const compile = proto_compile.compile;
const Thread = std.Thread;
pub fn main() anyerror!void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    var allocator = arena.allocator();
    var proto = try Thread.spawn(Thread.SpawnConfig{ .stack_size = 1024 * 1024 * 4 }, compile, .{allocator});
    proto.join();
}
