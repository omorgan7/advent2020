const input = String(require('fs').readFileSync("input.txt")).split("\n\n").map((passport) => passport.replace(/\n/g, " "))

const required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
const optional = ["cid"]


console.log(input.reduce((previous, passport) => {
  return previous + Number(required.every((field) => passport.search(field) != -1))
}, 0))