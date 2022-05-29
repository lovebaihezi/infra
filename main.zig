const std = @import("std");
const infra = @import("infra");

pub fn main() anyerror!void {
    var args = std.process.args();
    var args_iter = args.init();
    defer args_iter.deinit();
}
