const std = @import("std");

const MDIdx = @Vector(2, isize);
const MappingFn = fn (MDIdx, MDIdx) usize;
fn rowMajorMappingWithSentinel(idx: MDIdx, dimensions: MDIdx) usize {
    return @intCast(idx[1] * (dimensions[0] + 1) + idx[0]);
}

fn MDSlice(comptime T: type, comptime mapping: MappingFn) type {
    return struct {
        const Self = @This();
        pub const Idx = MDIdx;

        dimensions: Idx,
        buffer: [*]const T,

        pub fn newAlloc(dimensions: Idx, allocator: std.mem.Allocator) !Self {
            const bufferSize = dimensions[0] * dimensions[1];
            return Self{
                .buffer = try allocator.alloc(T, bufferSize),
                .dimensions = dimensions,
            };
        }

        pub fn fromRowMajorSliceWithSentinel(slice: []const T, rowSentinel: T) Self {
            const dimensions: Idx = .{
                @intCast(std.mem.indexOfScalar(T, slice, rowSentinel).?),
                @intCast(std.mem.count(T, slice, &[1]T{rowSentinel})),
            };
            return Self{ .dimensions = dimensions, .buffer = slice.ptr };
        }

        pub fn get(self: *const Self, idx: Idx) T {
            return self.buffer[
                mapping(
                    idx,
                    self.dimensions,
                )
            ];
        }

        pub fn tryGet(self: *const Self, idx: Idx) ?T {
            inline for (0..@typeInfo(Idx).Vector.len) |d| {
                if (idx[d] < 0 or idx[d] >= self.dimensions[d]) {
                    return null;
                }
            }
            return self.get(idx);
        }
    };
}

const Input = MDSlice(u8, rowMajorMappingWithSentinel);

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const input = try getInput(allocator);
    defer allocator.free(input[0].buffer[0..input[1]]);
    try std.io.getStdOut().writer().print("Part 1: {}\n", .{part1(&input[0])});
    try std.io.getStdOut().writer().print("Part 2: {}\n", .{part2(&input[0])});
}

fn getInput(allocator: std.mem.Allocator) !struct { Input, usize } {
    const bufferSize = 1024 * 20;
    var stdin = std.io.getStdIn().reader();
    const buffer = try stdin.readAllAlloc(allocator, bufferSize);
    return .{ Input.fromRowMajorSliceWithSentinel(buffer, '\n'), bufferSize };
}

pub fn part1(input: *const Input) u32 {
    const pattern = "XMAS";
    const directions = [_]Input.Idx{
        .{ -1, -1 },
        .{ -1, 0 },
        .{ -1, 1 },
        .{ 0, -1 },
        .{ 0, 1 },
        .{ 1, -1 },
        .{ 1, 0 },
        .{ 1, 1 },
    };

    var count: u32 = 0;
    const cols: usize = @intCast(input.dimensions[0]);
    const rows: usize = @intCast(input.dimensions[1]);
    for (0..rows) |y| {
        for (0..cols) |x| {
            const base: Input.Idx = .{ @bitCast(x), @bitCast(y) };
            if (input.get(base) != pattern[0]) {
                continue;
            }
            dirLoop: for (directions) |direction| {
                inline for (pattern, 0..pattern.len) |c, i| {
                    const offset = direction * @as(Input.Idx, @splat(i));
                    const idx = base + offset;
                    if (input.tryGet(idx) != c) {
                        continue :dirLoop;
                    }
                }
                count += 1;
            }
        }
    }
    return count;
}

pub fn part2(input: *const Input) u32 {
    const corners = [_]Input.Idx{ .{ -1, -1 }, .{ 1, -1 }, .{ 1, 1 }, .{ -1, 1 } };
    const patterns = [_][]const u8{ "MMSS", "SMMS", "SSMM", "MSSM" };
    const center = 'A';
    const cols: usize = @intCast(input.dimensions[0]);
    const rows: usize = @intCast(input.dimensions[1]);
    var count: u32 = 0;
    for (1..rows - 1) |y| {
        for (1..cols - 1) |x| {
            const base: Input.Idx = .{ @bitCast(x), @bitCast(y) };
            if (input.get(base) != center) {
                continue;
            }
            var cornerExpr: [4]u8 = undefined;
            for (corners, 0..cornerExpr.len) |corner, i| {
                cornerExpr[i] = input.get(base + corner);
            }
            for (patterns) |pattern| {
                if (std.mem.eql(u8, pattern, &cornerExpr)) {
                    count += 1;
                    break;
                }
            }
        }
    }
    return count;
}

test "mdslice mapping" {
    const grid = Input.fromRowMajorSliceWithSentinel(
        \\ABC
        \\DEF
        \\GHI
    , '\n');
    try std.testing.expectEqual('A', grid.get(.{ 0, 0 }));
    try std.testing.expectEqual('B', grid.get(.{ 1, 0 }));
    try std.testing.expectEqual('C', grid.get(.{ 2, 0 }));
    try std.testing.expectEqual('E', grid.get(.{ 1, 1 }));
    try std.testing.expectEqual('I', grid.get(.{ 2, 2 }));
    try std.testing.expectEqual('D', grid.get(.{ 0, 1 }));
    try std.testing.expectEqual('G', grid.get(.{ 0, 2 }));
}

test "part1 example" {
    const grid = Input.fromRowMajorSliceWithSentinel(
        \\MMMSXXMASM
        \\MSAMXMSMSA
        \\AMXSXMAAMM
        \\MSAMASMSMX
        \\XMASAMXAMM
        \\XXAMMXXAMA
        \\SMSMSASXSS
        \\SAXAMASAAA
        \\MAMMMXMMMM
        \\MXMXAXMASX
    , '\n');
    try std.testing.expectEqual(10, part1(&grid));
}
