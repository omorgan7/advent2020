const input = String(require('fs').readFileSync("input.txt")).split("\n\n").map((passport) => passport.replace(/\n/g, " "))

const required = [
  (passport) => {
    const matches = /byr:(\d{4})/.exec(passport)
    return matches ? matches[1] > 1919 && matches[1] < 2003 : false
  },
  (passport) => {
    const matches = /iyr:(\d{4})/.exec(passport)
    return matches ? matches[1] > 2009 && matches[1] < 2021 : false
  },
  (passport) => {
    const matches = /eyr:(\d{4})/.exec(passport)
    return matches ? matches[1] > 2019 && matches[1] < 2031 : false
  },
  (passport) => {
    const matches = /hgt:(\d+)(cm|in)/.exec(passport)
    if (!matches) return false

    if (matches[2] == "in") {
      return matches[1] > 58 && matches[1] < 77
    }
    return matches[1] > 149 && matches[1] < 194
  },
  (passport) => {
    return /hcl:(#[0-9a-f]{6})/.test(passport)
  },
  (passport) => {
    return /ecl:(amb|blu|brn|gry|grn|hzl|oth)/.test(passport)
  },
  (passport) => {
    return /pid:(\d{9}( |$))/.test(passport)
  } 
]

console.log(input.reduce((previous, passport) => {
  
  const valid = required.every((validator) => validator(passport))
  return previous + Number(valid)
}, 0))