const input = require('fs').readFileSync("input.txt", "utf8").split("\n")

const seatIds = new Set()

for (let i = 0; i < 127; ++i) {
    for (let j = 0; j < 8; ++j) {
        seatIds.add(j + 8 * i)
    }
}

input.forEach((current) => {
    let rowUpper = 127
    let rowLower = 0
    let colUpper = 7
    let colLower = 0
    let row, col
    for (const c of current) {
        switch(c) {
            case 'F': {
                rowUpper = Math.floor((rowUpper + rowLower) / 2)
                row = rowUpper
            } break
            case 'B': {
                rowLower = Math.ceil((rowUpper + rowLower) / 2)
                row = rowLower
            } break
            case 'L': {
                colUpper = Math.floor((colUpper + colLower) / 2)
                col = colUpper
            } break
            case 'R': {
                colLower = Math.ceil((colUpper + colLower) / 2)
                col = colLower
            } break
        }
    }
    seatIds.delete(row * 8 + col)
})

let remainingSeats = Array.from(seatIds)

let i = 1
for (; i < remainingSeats.length; ++i) {
    if (remainingSeats[i] - 1 != remainingSeats[i - 1]) {
        break
    }
}

console.log(remainingSeats[i])