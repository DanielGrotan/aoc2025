import Foundation

let input = try String(contentsOfFile: "input.txt", encoding: .utf8)

let rows =
    input
    .split(whereSeparator: \.isNewline)
    .map(String.init)

let chars = rows.map { Array($0) }

let separatorCols = chars[0].indices.filter { col in
    chars.allSatisfy { $0[col] == " " }
}

let boundaries = [-1] + separatorCols + [chars[0].count]

let columns = boundaries.indices.dropLast().map { i in
    let start = boundaries[i] + 1
    let end = boundaries[i + 1]

    return rows.map { row in
        let startIdx = row.index(row.startIndex, offsetBy: start)
        let endIdx = row.index(row.startIndex, offsetBy: end)
        let slice = row[startIdx..<endIdx]
        return Array(slice)
    }
}

let opTable: [Character: (initial: Int, combine: (Int, Int) -> Int)] = [
    "+": (0, +),
    "-": (0, -),
    "*": (1, *),
    "/": (1, /),
]

let total = columns.reduce(0) { acc, column in
    let values = column.dropLast()

    let numbers = values[0].indices.reversed().compactMap { i in
        let number = values.map { $0[i] }.filter { $0 != " " }.map(String.init).joined()
        return Int(number)
    }

    guard let lastRow = column.last, let op = lastRow.first else {
        return acc
    }
    let (initial, combine) = opTable[op]!

    return acc + numbers.reduce(initial, combine)
}
print(total)
