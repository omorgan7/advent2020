const input = require('fs').readFileSync("input.txt", "utf-8").split("\n")

const gold = new Set()

let setSize
do {
  setSize = [...gold.values()].length
  gold.add("shiny gold")
  input.forEach((line) => {
    [...gold.values()].forEach((bag) => {
      const matches = new RegExp(`^(.+) bags contain .+ ${bag} bag`).exec(line)
      if (matches) {
        gold.add(matches[1])
      }
    })
  })
} while(setSize != [...gold.values()].length)

console.log([...gold.values()].length - 1)