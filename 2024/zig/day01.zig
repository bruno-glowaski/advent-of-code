const std = @import("std");

const LocationID = u32;
const LocationIDSort = std.sort.asc(LocationID);
const Input = [2][]LocationID;

pub fn main() !void {
    const alloc = std.heap.page_allocator;
    const input = try parseInput(alloc);
    defer for (input) |list| alloc.free(list);

    try std.io.getStdOut().writer().print("Part 1: {}\n", .{try partOne(&input)});
    try std.io.getStdOut().writer().print("Part 2: {}\n", .{try partTwo(&input)});
}

fn parseInput(alloc: std.mem.Allocator) !Input {
    var list1 = std.ArrayList(LocationID).init(alloc);
    var list2 = std.ArrayList(LocationID).init(alloc);

    var stdin = std.io.getStdIn().reader();
    while (try stdin.readUntilDelimiterOrEofAlloc(alloc, '\n', 256)) |line| {
        var iter = std.mem.split(u8, std.mem.trim(u8, line, "\r"), "   ");
        try list1.append(try std.fmt.parseInt(LocationID, iter.next().?, 10));
        try list2.append(try std.fmt.parseInt(LocationID, iter.next().?, 10));
    }

    return .{ try list1.toOwnedSlice(), try list2.toOwnedSlice() };
}

fn partOne(input: *const Input) !u32 {
    const alloc = std.heap.page_allocator;
    const list1 = try alloc.dupe(LocationID, input.*[0]);
    const list2 = try alloc.dupe(LocationID, input.*[1]);

    std.mem.sort(
        LocationID,
        list1,
        {},
        LocationIDSort,
    );
    std.mem.sort(
        LocationID,
        list2,
        {},
        LocationIDSort,
    );

    var total: u64 = 0;
    for (list1, list2) |i, j| {
        total += @abs(@as(i64, i) - @as(i64, j));
    }
    return @intCast(total);
}

fn partTwo(input: *const Input) !u32 {
    const alloc = std.heap.page_allocator;
    const list1 = try alloc.dupe(LocationID, input.*[0]);
    const list2 = try alloc.dupe(LocationID, input.*[1]);

    var total: u32 = 0;
    for (list1) |i| {
        const frequency: u32 = @intCast(std.mem.count(LocationID, list2, &[_]LocationID{i}));
        total += frequency * i;
    }
    return total;
}
