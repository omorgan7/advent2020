const input = require('fs').readFileSync("input.txt", "utf-8").split("\n\n").map((line) => line.split("\n"))
console.log(input.reduce((prev, current) =>  {

    let c = {}
    current.forEach((person) => {
        person.split('').forEach((answer) => {
            if (c[answer]) {
                ++c[answer]
            }
            else {
                c[answer] = 1
            }
        })
    })
    return prev + Object.keys(c).reduce((p, k) => p + Number(c[k] == current.length), 0)
}, 0))