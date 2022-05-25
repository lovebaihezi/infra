const std = @import("std");
const infra = @import("infra");
const protobuf_file_generate = infra.proto_compile.compile;
const Thread = std.Thread;

pub fn main() anyerror!void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    var allocator = arena.allocator();
    try protobuf_file_generate(allocator);
}
