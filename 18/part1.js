const input = require('fs').readFileSync("18/input.txt", "utf-8").replace(/ /g, "").split("\n")

const evaluate = function(input) {
  let accumulator = 0
  let op = {
    "+": (a, b) => a + b,
    "*": (a, b) => a * b
  }
  let prevOp = '+'
  let prevNum

  for (let i = 0; i < input.length; ++i) {
    if (input[i] == '+') {
      prevOp = '+'
    }
    else if (input[i] == '*') {
      prevOp = '*'
    }
    else if (input[i] == '(') {
      const [retValue, newIndex] = evaluate(input.slice(i + 1))
      accumulator = op[prevOp](accumulator, retValue)
      i += newIndex
    }
    else if (input[i] == ')') {
      return [accumulator, i + 1]
    }
    else {
        accumulator = op[prevOp](accumulator, +input[i])
    }
  }
  return accumulator
}


console.log(input.reduce((acc, current) => acc + evaluate(current), 0))