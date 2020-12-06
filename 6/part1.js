const input = require('fs').readFileSync("input.txt", "utf-8").split("\n\n").map((line) => line.replace(/\n/g, ""))
console.log(input.reduce((prev, current) => prev + [...new Set(current)].length, 0))