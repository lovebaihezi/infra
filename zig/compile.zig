const std = @import("std");
const fs = std.fs;
const Dir = std.fs.Dir;
const Allocator = std.mem.Allocator;
const infra = @import("infra.zig");
const util = infra.util;
const which = util.which;
const getNameWithExt = util.getNameWithExt;

pub const ProtocError = error{
    ProtocNotFound,
};

pub const ProtocArgs = struct {
    include_path: []const u8,
    output_dir: []const u8,
    /// --cpp_out=OUT_DIR         0b0000000001  Generate C++ header and source.
    /// --csharp_out=OUT_DIR      0b0000000010  Generate C# source file.
    /// --java_out=OUT_DIR        0b0000000100  Generate Java source file.
    /// --js_out=OUT_DIR          0b0000001000  Generate JavaScript source.
    /// --kotlin_out=OUT_DIR      0b0000010000  Generate Kotlin file.
    /// --objc_out=OUT_DIR        0b0000100000  Generate Objective-C header and source.
    /// --php_out=OUT_DIR         0b0001000000  Generate PHP source file.
    /// --pyi_out=OUT_DIR         0b0010000000  Generate python pyi stub.
    /// --python_out=OUT_DIR      0b0100000000  Generate Python source file.
    /// --ruby_out=OUT_DIR        0b1000000000  Generate Ruby source file.
    /// will generate all be default
    option: u16 = 0b1111111111,
};

/// generate args for ChildProcess to execv, returned value should be owned by caller
fn generateArgs(allocator: Allocator, arg: ProtocArgs) anyerror![]const []const u8 {
    std.debug.assert(arg.option != 0);
    const optional_protoc_path = try which(allocator, "protoc");
    const arr = [_][]const u8{
        "--cpp_out=",
        "--csharp_out=",
        "--java_out=",
        "--js_out=",
        "--kotlin_out=",
        "--objc_out=",
        "--php_out=",
        "--pyi_out=",
        "--python_out=",
        "--ruby_out=",
    };
    if (optional_protoc_path) |protoc_path| {
        var buf = try std.ArrayListUnmanaged([]const u8).initCapacity(allocator, 16);
        const include_path = try std.mem.join(allocator, "", &[_][]const u8{ "--proto_path=", arg.include_path });
        try buf.append(allocator, protoc_path);
        try buf.append(allocator, include_path);
        var value = arg.option;
        comptime var i: u6 = 0;
        inline while (i < 10) : (i += 1) {
            const v = value & 1;
            value = value >> 1;
            if (v == 1) {
                const output_dir = try std.mem.join(allocator, "", &[_][]const u8{ arr[i], arg.output_dir });
                try buf.append(allocator, output_dir);
            }
        }
        return buf.toOwnedSlice(allocator);
    } else {
        return ProtocError.ProtocNotFound;
    }
}

fn argsDeinit(allocator: Allocator, args: []const []const u8) void {
    for (args) |arg| {
        allocator.free(arg);
    }
    allocator.free(args);
}

test "generate protoc args" {
    var allocator = std.testing.allocator;
    const args = try generateArgs(allocator, ProtocArgs{
        .include_path = "./proto",
        .output_dir = "cpp/proto",
        .option = 0b1000000000,
    });
    defer argsDeinit(allocator, args);
    std.debug.print("\n", .{});
    for (args) |arg| {
        std.debug.print("{s}\n", .{arg});
    }
}

test "bit" {
    {
        var value: usize = 0b100_000_000_1;
        comptime var i: u6 = 0;
        var count: usize = 0;
        inline while (i < 10) : (i += 1) {
            const v = value & 1;
            value = value >> 1;
            if (v == 1) {
                count += 1;
            }
        }
        try std.testing.expect(count == 2);
    }
    {
        // TODO: bitwise
        var value: usize = 0b100_010_100_1;
        comptime var i: u6 = 0;
        var count: usize = 0;
        inline while (i < 10) : (i += 1) {
            const v = value & 1;
            value = value >> 1;
            if (v == 1) {
                count += 1;
            }
        }
        std.debug.print("\n", .{});
        try std.testing.expect(count == 4);
    }
}

fn execProtoc(allocator: Allocator, args: []const []const u8) !void {
    var child_process = try std.ChildProcess.init(args, allocator);
    defer child_process.deinit();
    const out = child_process.stdout;
    const err = child_process.stderr;
    const term = try child_process.spawnAndWait();
    if (out) |child_out| {
        if (err) |child_err| {
            var buf: [4096]u8 = undefined;
            var read_n = try child_out.read(&buf);
            while (read_n != 0) {
                std.log.info("{s}", .{buf[0..read_n]});
                read_n = try child_out.read(&buf);
            }
            read_n = try child_err.read(&buf);
            while (read_n != 0) {
                std.log.err("{s}", .{buf[0..read_n]});
                read_n = try child_err.read(&buf);
            }
        }
    }
    if (term.Exited != 0) {
        std.log.err("protoc exit with {d}", .{term.Exited});
    }
}
pub fn compile(allocator: Allocator) !void {
    const cwd = fs.cwd();
    var root = try cwd.openDir("./proto", std.fs.Dir.OpenDirOptions{ .iterate = true });
    var walk = try root.walk(allocator);
    defer walk.deinit();
    defer root.close();
    const args = try generateArgs(allocator, ProtocArgs{
        .include_path = try std.fs.realpathAlloc(allocator, "./proto"),
        .output_dir = try std.fs.realpathAlloc(allocator, "./cpp/proto"),
        .option = 0b000000001,
    });
    defer argsDeinit(allocator, args);
    var buf = try std.ArrayListUnmanaged([]const u8).initCapacity(allocator, 128);
    for (args) |arg| {
        try buf.append(allocator, arg);
    }
    while (try walk.next()) |entry| {
        if (entry.kind == std.fs.Dir.Entry.Kind.File) {
            const name_with_ext = getNameWithExt(entry.path, std.fs.path.sep);
            if (name_with_ext.len > "proto".len) {
                const ext = std.fs.path.extension(name_with_ext);
                if (std.mem.eql(u8, ext, ".proto")) {
                    std.log.info("proto file {s} will be compiled", .{entry.path});
                    var buffer = try allocator.alloc(u8, entry.path.len);
                    std.mem.copy(u8, buffer, entry.path);
                    try buf.append(allocator, buffer);
                }
            }
        }
    }
    const arg = buf.toOwnedSlice(allocator);
    defer {
        for (arg[args.len..]) |a| {
            allocator.free(a);
        }
    }
    defer allocator.free(arg);
    try execProtoc(allocator, arg);
}
