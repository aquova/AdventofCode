package.path = package.path .. ";../?.lua"
local Intcode = require("intcode")
local Utils = require("utils")

function main()
    local filename = "input.txt"
    local f = Utils.load_file(filename)
    local opcodes = Utils.split(f[1], ",")

    local ic = Intcode:new(opcodes)
    print(ic:execute(1))
end

main()
