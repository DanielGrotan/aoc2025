const std = @import("std");

const IdRange = struct {
    start: usize,
    end: usize,

    const Self = @This();

    pub fn parse(string: []const u8) !Self {
        var split = std.mem.splitSequence(u8, string, "-");

        const start = split.next().?;
        const start_value = try std.fmt.parseInt(usize, start, 10);

        const end = split.next().?;
        const end_value = try std.fmt.parseInt(usize, end, 10);

        return .{
            .start = start_value,
            .end = end_value,
        };
    }

    pub fn sumWrongIds(self: *const Self) !usize {
        var sum: usize = 0;
        var buf: [20]u8 = undefined;

        for (self.start..self.end + 1) |number| {
            const id = try std.fmt.bufPrint(&buf, "{}", .{number});

            if (isWrongId(id)) {
                sum += number;
            }
        }

        return sum;
    }
};

fn isWrongId(id: []const u8) bool {
    for (1..id.len / 2 + 1) |pattern_length| {
        if (isRepeatingPattern(id, pattern_length)) {
            return true;
        }
    }

    return false;
}

fn isRepeatingPattern(id: []const u8, pattern_length: usize) bool {
    if (id.len % pattern_length != 0) {
        return false;
    }

    for (0..id.len / pattern_length - 1) |offset| {
        const start = offset * pattern_length;
        const mid = start + pattern_length;
        const end = mid + pattern_length;

        if (!std.mem.eql(u8, id[start..mid], id[mid..end])) {
            return false;
        }
    }

    return true;
}

fn processInput(reader: *std.Io.Reader) !void {
    var sum: usize = 0;

    while (try reader.takeDelimiter(',')) |pair| {
        const trimmed = std.mem.trim(u8, pair, " \t\r\n");
        const id_range = try IdRange.parse(trimmed);

        const sum_wrong_ids = try id_range.sumWrongIds();
        sum += sum_wrong_ids;
    }

    std.debug.print("Sum: {}\n", .{sum});
}

pub fn main() !void {
    const file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    const stat = try file.stat();
    const file_size = stat.size;

    const allocator = std.heap.page_allocator;
    const buffer = try allocator.alloc(u8, file_size);
    defer allocator.free(buffer);

    var reader = file.reader(buffer);
    try processInput(&reader.interface);
}
