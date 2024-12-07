const std = @import("std");

const PageNumber = u32;

const PageOrderingRule = struct { PageNumber, PageNumber };

const MAX_PAGE = 100;
const MAX_SEQUENCE = 24;
const UPDATE_DELIMITER: PageNumber = 0;

const Input = struct {
    orderingRules: []const PageOrderingRule,
    updates: []const PageNumber,
    fn clone(self: Input, allocator: std.mem.Allocator) Input {
        return .{
            .orderingRules = allocator.dupe(PageOrderingRule, self.orderingRules),
            .updates = allocator.dupe(PageNumber, self.updates),
        };
    }
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const input = try parseInput(allocator);
    defer deinitInput(&input, allocator);
    try std.io.getStdOut().writer().print("Part 1: {!}\n", .{part1(&input, allocator)});
    try std.io.getStdOut().writer().print("Part 2: {!}\n", .{part2(&input, allocator)});
}

fn parseInput(allocator: std.mem.Allocator) !Input {
    var orderingRules = try std.ArrayList(PageOrderingRule).initCapacity(allocator, 1176);
    var updates = try std.ArrayList(PageNumber).initCapacity(allocator, 176 * MAX_SEQUENCE);
    const buffer = try allocator.alloc(u8, 69);
    var stdin = std.io.getStdIn().reader();
    while (true) {
        const line = try stdin.readUntilDelimiter(buffer, '\n');
        if (line.len == 0) {
            break;
        }
        var iter = std.mem.splitScalar(u8, line, '|');
        const before = try std.fmt.parseInt(PageNumber, iter.next().?, 10);
        const after = try std.fmt.parseInt(PageNumber, iter.next() orelse return error.NoDependent, 10);
        try orderingRules.append(.{ before, after });
    }
    while (try stdin.readUntilDelimiterOrEof(buffer, '\n')) |line| {
        if (line.len == 0) {
            break;
        }
        var pageIter = std.mem.splitScalar(u8, line, ',');
        while (pageIter.next()) |pageNumber| {
            try updates.append(try std.fmt.parseInt(PageNumber, pageNumber, 10));
        }
        try updates.append(UPDATE_DELIMITER);
    }
    _ = updates.pop();
    return .{ .orderingRules = try orderingRules.toOwnedSlice(), .updates = try updates.toOwnedSlice() };
}

fn deinitInput(input: *const Input, allocator: std.mem.Allocator) void {
    allocator.free(input.orderingRules);
    allocator.free(input.updates);
}

pub fn part1(input: *const Input, _: std.mem.Allocator) !u32 {
    const outbound = generateOutboundGraph(MAX_PAGE, input.orderingRules);
    var middleSum: u32 = 0;
    var updateIter = std.mem.splitScalar(PageNumber, input.updates, UPDATE_DELIMITER);
    updateIter: while (updateIter.next()) |update| {
        var prevPages = PageSet.initEmpty();
        for (update) |page| {
            if (outbound[page].intersectWith(prevPages).count() > 0) {
                continue :updateIter;
            }
            prevPages.set(page);
        }
        const middle = update[update.len / 2];
        middleSum += middle;
    }
    return middleSum;
}

pub fn part2(input: *const Input, _: std.mem.Allocator) !u32 {
    const outbound = generateOutboundGraph(MAX_PAGE, input.orderingRules);
    const inbound = generateInboundGraph(MAX_PAGE, input.orderingRules);
    var middleSum: u32 = 0;
    var updateIter = std.mem.splitScalar(PageNumber, input.updates, UPDATE_DELIMITER);
    while (updateIter.next()) |update| {
        var nodes = PageSet.initEmpty();
        var accept = false;
        for (update) |page| {
            if (outbound[page].intersectWith(nodes).count() > 0) {
                accept = true;
            }
            nodes.set(page);
        }

        if (!accept) {
            continue;
        }

        var subinbound = inbound;
        var noInbound = PageSet.initEmpty();
        var pageIter = nodes.iterator(.{});
        while (pageIter.next()) |page| {
            subinbound[page] = subinbound[page].intersectWith(nodes);
            if (subinbound[page].count() == 0) {
                noInbound.set(page);
            }
        }

        var newSequence = try std.BoundedArray(PageNumber, MAX_SEQUENCE).init(0);
        while (noInbound.toggleFirstSet()) |page| {
            try newSequence.append(@intCast(page));
            var outboundIter = outbound[page].iterator(.{});
            while (outboundIter.next()) |next| {
                if (!nodes.isSet(next)) {
                    continue;
                }
                subinbound[next].unset(page);
                if (subinbound[next].findFirstSet() == null) {
                    noInbound.set(next);
                }
            }
        }

        middleSum += newSequence.get(newSequence.len / 2);
    }
    return middleSum;
}

const PageSet = std.StaticBitSet(MAX_PAGE);
const AdjacencyGraph = [MAX_PAGE]PageSet;

fn generateOutboundGraph(comptime P: usize, rules: []const PageOrderingRule) AdjacencyGraph {
    var graph = initEmptyGraph(P);
    for (rules) |rule| {
        graph[rule[0]].set(rule[1]);
    }
    return graph;
}

fn generateInboundGraph(comptime P: usize, rules: []const PageOrderingRule) AdjacencyGraph {
    var graph = initEmptyGraph(P);
    for (rules) |rule| {
        graph[rule[1]].set(rule[0]);
    }
    return graph;
}

fn initEmptyGraph(comptime P: usize) AdjacencyGraph {
    var graph: AdjacencyGraph = undefined;
    for (0..P) |i| {
        graph[i] = PageSet.initEmpty();
    }
    return graph;
}

const exampleInput = Input{
    .orderingRules = &[_]PageOrderingRule{
        .{ 47, 53 },
        .{ 97, 13 },
        .{ 97, 61 },
        .{ 97, 47 },
        .{ 75, 29 },
        .{ 61, 13 },
        .{ 75, 53 },
        .{ 29, 13 },
        .{ 97, 29 },
        .{ 53, 29 },
        .{ 61, 53 },
        .{ 97, 53 },
        .{ 61, 29 },
        .{ 47, 13 },
        .{ 75, 47 },
        .{ 97, 75 },
        .{ 47, 61 },
        .{ 75, 61 },
        .{ 47, 29 },
        .{ 75, 13 },
        .{ 53, 13 },
    },
    .updates = &[_]PageNumber{
        75,
        47,
        61,
        53,
        29,
        UPDATE_DELIMITER,
        97,
        61,
        53,
        29,
        13,
        UPDATE_DELIMITER,
        75,
        29,
        13,
        UPDATE_DELIMITER,
        75,
        97,
        47,
        61,
        53,
        UPDATE_DELIMITER,
        61,
        13,
        29,
        UPDATE_DELIMITER,
        97,
        13,
        75,
        29,
        47,
    },
};

test "day 5 part 1" {
    try std.testing.expectEqual(143, try part1(&exampleInput, std.testing.allocator));
}

test "day 5 part 2" {
    try std.testing.expectEqual(123, try part2(&exampleInput, std.testing.allocator));
}
