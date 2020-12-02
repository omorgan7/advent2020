'use strict'

const output = String(require('fs').readFileSync('input.txt'))
                .split('\n')
                .filter((line) => {
                  const matches = /(\d+)\-(\d+) ([a-z]): ([a-z]+)/.exec(line)
                  const letter = matches[3]
                  const input = matches[4].split('')
                  const matchFirst = input[matches[1] - 1] == letter
                  const matchSecond = input[matches[2] - 1] == letter
                  if (matchFirst && matchSecond) {
                    return false
                  }
                  return matchFirst || matchSecond
                })
                .length

console.log(output)