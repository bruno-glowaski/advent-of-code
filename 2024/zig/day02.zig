const std = @import("std");

const ReportLevel = i32;
const Input = []ReportLevel;
const TERMINATOR = 0;

pub fn main() !void {
    const alloc = std.heap.page_allocator;
    const input = try parseInput(alloc);

    try std.io.getStdOut().writer().print("Part 1: {}\nPart 2: {}\n", .{ partOne(&input), partTwo(&input) });
}

pub fn parseInput(alloc: std.mem.Allocator) !Input {
    var reports = std.ArrayList(ReportLevel).init(alloc);

    var stdin = std.io.getStdIn().reader();
    while (try stdin.readUntilDelimiterOrEofAlloc(alloc, '\n', 256)) |line| {
        var levelIter = std.mem.split(u8, std.mem.trim(u8, line, "\r"), " ");
        while (levelIter.next()) |level| {
            try reports.append(try std.fmt.parseInt(ReportLevel, level, 10));
        }
        try reports.append(TERMINATOR);
    }
    _ = reports.pop();
    return reports.toOwnedSlice();
}

pub fn partOne(input: *const Input) u32 {
    var safeReportCount: u32 = 0;

    var reportIter = std.mem.splitScalar(ReportLevel, input.*, TERMINATOR);
    while (reportIter.next()) |report| {
        if (isSafe(report)) {
            safeReportCount += 1;
        }
    }

    return safeReportCount;
}

pub fn partTwo(input: *const Input) u32 {
    var safeReportCount: u32 = 0;

    var reportIter = std.mem.splitScalar(ReportLevel, input.*, TERMINATOR);
    reports: while (reportIter.next()) |report| {
        if (isSafe(report)) {
            safeReportCount += 1;
            continue;
        }
        var buffer: [8]ReportLevel = undefined;
        const subreport = buffer[0 .. report.len - 1];
        for (0..report.len) |i| {
            var k: u8 = 0;
            for (0..report.len) |j| {
                if (j == i) {
                    continue;
                }
                subreport[k] = report[j];
                k += 1;
            }
            if (isSafe(subreport)) {
                safeReportCount += 1;
                continue :reports;
            }
        }
    }

    return safeReportCount;
}

fn isSafe(report: []const ReportLevel) bool {
    var delta0 = report[1] - report[0];
    if (isUnsafeDelta(delta0)) {
        return false;
    }
    var sequenceIter = std.mem.window(ReportLevel, report, 3, 1);
    while (sequenceIter.next()) |sequence| {
        delta0 = sequence[1] - sequence[0];
        const delta1 = sequence[2] - sequence[1];
        if (isUnsafeDelta(delta1) or (std.math.sign(delta0) != std.math.sign(delta1))) {
            return false;
        }
    }
    return true;
}

// pub fn partTwo(input: *const Input) u32 {
//     var safeReportCount: u32 = 0;
//
//     var reportIter = std.mem.splitScalar(ReportLevel, input.*, TERMINATOR);
//     reports: while (reportIter.next()) |report| {
//         var didSkip = false;
//         var i: usize = 1;
//         var delta1 = report[i] - report[i - 1]; // current - last
//
//         if (isUnsafeDelta(delta1)) {
//             // Skip first level (no bound check because we know reports have at least 5 levels).
//             didSkip = true;
//             i += 1;
//             delta1 = report[i] - report[i - 1];
//             if (isUnsafeDelta(delta1)) {
//                 // Skip second level.
//                 delta1 = report[i + 1] - report[i - 1];
//                 if (isUnsafeDelta(delta1)) {
//                     continue;
//                 }
//             }
//         }
//         i += 1;
//
//         while (i < report.len) {
//             const delta0 = delta1; // last - second last. We need to use the last value of delta1 to account for potential skips.
//             delta1 = report[i] - report[i - 1];
//             if (!isUnsafeDelta(delta1) and std.math.sign(delta0) == std.math.sign(delta1)) {
//                 // Level i does not introduce unsafety.
//                 i += 1;
//                 continue;
//             }
//             if (didSkip) {
//                 // We have already skipped a level.
//                 continue :reports;
//             }
//             // Skip the next level.
//             i += 1;
//             didSkip = true;
//             if (i == report.len) {
//                 // We have reached the end and we can just ignore the last item.
//                 break;
//             }
//             delta1 = report[i] - report[i - 2];
//             if (isUnsafeDelta(delta1) or std.math.sign(delta0) != std.math.sign(delta1)) {
//                 _ = std.io.getStdOut().writer().print("Skipping report {any}\n", .{report}) catch 0;
//                 // The next level still makes it unsafe.
//                 continue :reports;
//             }
//             i += 1;
//         }
//
//         safeReportCount += 1;
//     }
//
//     return safeReportCount;
// }

fn isUnsafeDelta(delta: i32) bool {
    const abs = @abs(delta);
    return abs == 0 or abs > 3;
}
