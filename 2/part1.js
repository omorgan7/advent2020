'use strict'

const output = String(require('fs').readFileSync('input.txt'))
                .split('\n')
                .filter((line) => {
                  const matches = /(\d+)\-(\d+) ([a-z]): ([a-z]+)/.exec(line)
                  const letter = matches[3]
                  const input = matches[4]

                  const count = input.split('').reduce((count, c) => count + Number(c == letter), 0)

                  return count >= matches[1] && count <= matches[2]
                })
                .length

console.log(output)