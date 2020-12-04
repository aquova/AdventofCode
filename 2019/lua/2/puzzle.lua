-- AOC 2019 Day 2
-- Simulating a computer
package.path = package.path .. ";../?.lua"
local Utils = require("utils")

function main()
    local filename = "input.txt"
    local f = Utils.load_file(filename)
    local opcodes = Utils.split(f[1], ",")

    -- Part 1:
    -- print(intcode(opcodes, 12, 2))

    -- Part 2:
    local target = 19690720
    local done = false
    for i = 0, 100 do
        if done then break end
        for j = 0, 100 do
            local val = intcode(opcodes, i, j)
            if val == target then
                print(100 * i + j)
                done = true
                break
            end
        end
    end
end

function intcode(master, start_val1, start_val2)
    local opcodes = Utils.copy_table(master)
    opcodes[1 + 1] = start_val1
    opcodes[2 + 1] = start_val2

    local pc = 1
    while true do
        local op = tonumber(opcodes[pc])

        if op == 99 then
            break
        elseif op == 1 then
            local operand_addr1 = opcodes[pc + 1] + 1 -- +1 because Lua is 1-indexed
            local operand_addr2 = opcodes[pc + 2] + 1
            local sum = opcodes[operand_addr1] + opcodes[operand_addr2]
            local addr = opcodes[pc + 3] + 1
            opcodes[addr] = sum
            pc = pc + 4
        elseif op == 2 then
            local operand_addr1 = opcodes[pc + 1] + 1 -- +1 because Lua is 1-indexed
            local operand_addr2 = opcodes[pc + 2] + 1
            local prod = opcodes[operand_addr1] * opcodes[operand_addr2]
            local addr = opcodes[pc + 3] + 1
            opcodes[addr] = prod
            pc = pc + 4
        else
            print(op)
            print(pc)
            break
        end
    end

    return opcodes[1]
end

main()
