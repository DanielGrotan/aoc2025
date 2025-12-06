import Foundation

let input = try String(contentsOfFile: "input.txt", encoding: .utf8)

let lines = input.split(whereSeparator: \.isNewline).map(String.init)
let numberLines = lines.dropLast()
let operatorLine = lines.last!

let numbers = numberLines.map {
    $0.split { $0.isWhitespace }.compactMap { Int($0) }
}

let transposedNumbers = numbers[0].indices.map { col in
    numbers.map { row in row[col] }
}

let operators =
    operatorLine
    .split(whereSeparator: \.isWhitespace)
    .compactMap(\.first)

let opTable: [Character: (initial: Int, combine: (Int, Int) -> Int)] = [
    "+": (0, +),
    "-": (0, -),
    "*": (1, *),
    "/": (1, /),
]

let total = zip(transposedNumbers, operators).reduce(0) { acc, pair in
    let (values, op) = pair
    let (initial, combine) = opTable[op]!
    return acc + values.reduce(initial, combine)
}
print(total)
