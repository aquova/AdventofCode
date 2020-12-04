-- AOC 2019 Day 7
-- Simulating a computer
package.path = package.path .. ";../?.lua"
local Intcode = require("intcode")
local Utils = require("utils")

function main()
    local filename = "test.txt"
    local f = Utils.load_file(filename)
    local opcodes = Utils.split(f[1], ",")

    local perms = {}
    gen_perms(perms, {9, 8, 7, 6, 5}, 5)

    local cpus = {}
    for i = 1, 5 do
        cpus[i] = Intcode:new(opcodes)
    end

    local input = 0
    for i = 1, 5 do
        input = cpus[i]:execute(input)
    end
end

-- Algorithm from here: https://www.lua.org/pil/9.3.html
function gen_perms(output, a, n)
    if n == 0 then
        table.insert(output, Utils.copy_table(a))
    else
        for i = 1, n do
            a[n], a[i] = a[i], a[n]
            gen_perms(output, a, n - 1)
            a[n], a[i] = a[i], a[n]
        end
    end
end

main()
