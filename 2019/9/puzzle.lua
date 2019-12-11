-- Advent of Code 2019 Day 9
package.path = package.path .. ";../?.lua"
local Intcode = require("intcode")
local Utils = require("utils")

function main()
    local FILENAME = "input.txt"
    local f = Utils.load_file(FILENAME)
    local opcodes = Utils.split(f[1], ",")
    local machine = Intcode:new(opcodes)
    machine:execute(1)
end

main()
