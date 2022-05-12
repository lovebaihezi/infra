const std = @import("std");

pub fn build(b: *std.build.Builder) !void {
    // Standard target options allows the person running `zig build` to choose
    // what target to build for. Here we do not override the defaults, which
    // means any target is allowed, and the default is native. Other options
    // for restricting supported target set are available.
    const target = b.standardTargetOptions(.{});

    // Standard release options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall.
    const mode = b.standardReleaseOptions();
    const exe = b.addExecutable("build", "main.zig");
    exe.addPackagePath("infra", "zig/infra.zig");
    exe.setTarget(target);
    exe.setBuildMode(mode);
    exe.install();

    const run_cmd = exe.run();
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    const main = b.addTest("main.zig");
    main.addPackagePath("infra", "zig/infra.zig");
    main.setTarget(target);
    main.setBuildMode(mode);

    const compile = b.addTest("proto/compile.zig");
    compile.addPackagePath("infra", "zig/infra.zig");
    compile.setTarget(target);
    compile.setBuildMode(mode);

    const util = b.addTest("zig/util/util.zig");
    util.addPackagePath("infra", "zig/infra.zig");
    util.setTarget(target);
    util.setBuildMode(mode);

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&main.step);
    test_step.dependOn(&compile.step);
    test_step.dependOn(&util.step);
}
