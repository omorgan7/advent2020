const input = require('fs').readFileSync("input.txt", "utf8").split("\n")

console.log(input.reduce((prev, current) => {
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
    return Math.max(prev, row * 8 + col)
}, 0))