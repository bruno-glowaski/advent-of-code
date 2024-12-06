const std = @import("std");
const re = @cImport(@cInclude("lib/regez.h"));

pub fn main() !void {
    const alloc = std.heap.page_allocator;
    const input = try readInput(alloc);
    defer alloc.free(input);
    try std.io.getStdOut().writer().print("Part 1: {}\nPart 2: {}\n", .{ try part1(input), try part2(input) });
}

fn readInput(alloc: std.mem.Allocator) ![]u8 {
    var buffer = try alloc.alloc(u8, 1024 * 20);
    var stdin = std.io.getStdIn().reader();
    const length = try stdin.readAll(buffer);
    buffer[length] = 0;
    return buffer[0 .. length + 1];
}

pub fn part1(input: []const u8) !i64 {
    var regex: re.regex_t = undefined;
    var captures: [3]re.regmatch_t = undefined;
    _ = re.regcomp(&regex, "mul(\\([0-9]\\+\\),\\([0-9]\\+\\))", 0);
    defer re.regfree(&regex);

    _ = re.regexec(&regex, input.ptr, captures.len, &captures, 0);
    var i: usize = 0;
    var acc: i64 = 0;
    while (i < input.len) {
        const remainder = input[i..];
        if (re.regexec(&regex, remainder.ptr, captures.len, &captures, 0) != 0) {
            i += 1;
            continue;
        }
        const a = try std.fmt.parseInt(i64, getCapture(remainder, captures[1]), 10);
        const b = try std.fmt.parseInt(i64, getCapture(remainder, captures[2]), 10);
        acc += a * b;
        i += @intCast(captures[0].rm_eo);
    }
    return acc;
}

pub fn part2(input: []const u8) !i64 {
    var regex: re.regex_t = undefined;
    var captures: [3]re.regmatch_t = undefined;
    _ = re.regcomp(&regex, "do()\\|don't()\\|mul(\\([0-9]\\+\\),\\([0-9]\\+\\))", 0);
    defer re.regfree(&regex);

    _ = re.regexec(&regex, input.ptr, captures.len, &captures, 0);
    var enabled = true;
    var i: usize = 0;
    var acc: i64 = 0;
    while (i < input.len) {
        const remainder = input[i..];
        if (re.regexec(&regex, remainder.ptr, captures.len, &captures, 0) != 0) {
            i += 1;
            continue;
        }
        i += @intCast(captures[0].rm_eo);
        const expr = getCapture(remainder, captures[0]);
        if (std.mem.eql(u8, expr, "do()")) {
            enabled = true;
            continue;
        }
        if (std.mem.eql(u8, expr, "don't()")) {
            enabled = false;
            continue;
        }
        if (!enabled) {
            continue;
        }
        const a = try std.fmt.parseInt(i64, getCapture(remainder, captures[1]), 10);
        const b = try std.fmt.parseInt(i64, getCapture(remainder, captures[2]), 10);
        acc += a * b;
    }
    return acc;
}

fn getCapture(input: []const u8, match: re.regmatch_t) []const u8 {
    return input[@intCast(match.rm_so)..@intCast(match.rm_eo)];
}

test "part1 example" {
    // try std.testing.expectEqual(8, try part1("mul(2,4)"));
    try std.testing.expectEqual(161, try part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"));
}
