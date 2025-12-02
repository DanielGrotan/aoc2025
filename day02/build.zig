const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const part1 = b.addExecutable(.{
        .name = "part1",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/part1.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    b.installArtifact(part1);

    const run_part1 = b.addRunArtifact(part1);
    const step_part1 = b.step("part1", "Run part 1");
    step_part1.dependOn(&run_part1.step);

    const part2 = b.addExecutable(.{
        .name = "part2",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/part2.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    b.installArtifact(part2);

    const run_part2 = b.addRunArtifact(part2);
    const step_part2 = b.step("part2", "Run part 2");
    step_part2.dependOn(&run_part2.step);
}
