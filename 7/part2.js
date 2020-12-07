const input = require('fs').readFileSync("input.txt", "utf-8").split("\n")

const bags = {}
input.forEach((line) => {
    const matches = new RegExp(`^(.+) bags contain (.+)`).exec(line)
    if (!matches) {
      return
    }

    const contents = matches[2].split(',')
    bags[matches[1]] = {}
    contents.forEach((bag) => {
      const bagMatch = /(\d) (.+) bag/.exec(bag)
      if (!bagMatch) {
        return null
      }
      bags[matches[1]][bagMatch[2]] = +bagMatch[1]
    })
})

var count = 0
const search = function(bag, multiplier) {
  count += Object.values(bags[bag]).reduce((prev, v) => {
    return prev + multiplier * v
  }, 0)

  Object.keys(bags[bag]).forEach((b) => {search(b, multiplier * bags[bag][b])})
}

search("shiny gold", 1)

console.log(count)