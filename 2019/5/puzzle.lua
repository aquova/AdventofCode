-- AOC 2019 Day 2
-- Simulating a computer
package.path = package.path .. ";../?.lua"
local Utils = require("utils")

function main()
    local filename = "test.txt"
    local f = Utils.load_file(filename)
    local opcodes = Utils.split(f[1], ",")

    intcode(opcodes)
end

function intcode(master)
    local opcodes = Utils.copy_table(master)

    local pc = 1
    while true do
        local op = tonumber(opcodes[pc])
        local index = op % 100

        if index == 99 then
            break
        elseif index == 1 then
            local p1, p2, p3 = parse_opcode(op)
            -- +1 because Lua is 1-indexed
            local operand_addr1 = opcodes[pc + 1]
            local operand_addr2 = opcodes[pc + 2]
            local operand1 = p1 and opcodes[operand_addr1 + 1] or operand_addr1
            local operand2 = p2 and opcodes[operand_addr2 + 1] or operand_addr2

            local sum = operand1 + operand2
            -- p3 is always true, currently
            local addr = opcodes[pc + 3]
            opcodes[addr + 1] = sum
            pc = pc + 4
        elseif index == 2 then
            local p1, p2, p3 = parse_opcode(op)

            local operand_addr1 = opcodes[pc + 1]
            local operand_addr2 = opcodes[pc + 2]
            local operand1 = p1 and opcodes[operand_addr1 + 1] or operand_addr1
            local operand2 = p2 and opcodes[operand_addr2 + 1] or operand_addr2

            local prod = operand1 * operand2

            local addr = opcodes[pc + 3]
            opcodes[addr + 1] = prod
            pc = pc + 4
        elseif index == 3 then
            -- I'm not going to bother with I/O error handling. Assume user gives a valid input
            print("Input: ")
            local input = io.read()
            local addr = opcodes[pc + 1]
            opcodes[addr + 1] = tonumber(input)
            pc = pc + 2
        elseif index == 4 then
            local p1, _, _ = parse_opcode(op)
            local addr = opcodes[pc + 1]
            local outstr = p1 and opcodes[addr + 1] or addr
            print("Output: "..outstr)
            pc = pc + 2
        elseif index == 5 then
            local p1, p2, _ = parse_opcode(op)
            local addr1 = opcodes[pc + 1]
            local addr2 = opcodes[pc + 2]

            local jmp = p1 and opcodes[addr1 + 1] or addr1
            local target = p2 and opcodes[addr2 + 1] or addr2
            if tonumber(jmp) ~= 0 then
                pc = tonumber(target)
            else
                pc = pc + 3
            end
        elseif index == 6 then
            local p1, p2, _ = parse_opcode(op)
            local addr1 = opcodes[pc + 1]
            local addr2 = opcodes[pc + 2]

            local jmp = p1 and opcodes[addr1 + 1] or addr1
            local target = p2 and opcodes[addr2 + 1] or addr2
            if tonumber(jmp) == 0 then
                pc = tonumber(target)
            else
                pc = pc + 3
            end
        elseif index == 7 then
            local p1, p2, p3 = parse_opcode(op)
            local addr1 = opcodes[pc + 1]
            local addr2 = opcodes[pc + 2]
            local addr3 = opcodes[pc + 3]

            local param1 = p1 and opcodes[addr1 + 1] or addr1
            local param2 = p2 and opcodes[addr2 + 1] or addr2
            -- local param3 = p3 and opcodes[addr3 + 1] or addr3
            opcodes[addr3 + 1] = (tonumber(param1) < tonumber(param2)) and 1 or 0
            pc = pc + 4
        elseif index == 8 then
            local p1, p2, p3 = parse_opcode(op)
            local addr1 = opcodes[pc + 1]
            local addr2 = opcodes[pc + 2]
            local addr3 = opcodes[pc + 3]

            local param1 = p1 and opcodes[addr1 + 1] or addr1
            local param2 = p2 and opcodes[addr2 + 1] or addr2
            -- local param3 = p3 and opcodes[addr3 + 1] or addr3
            opcodes[addr3 + 1] = (tonumber(param1) == tonumber(param2)) and 1 or 0
            pc = pc + 4
        else
            print("Invalid opcode.")
            print(op)
            print(pc)
            break
        end
    end

    return opcodes[1]
end

function parse_opcode(op)
    -- Opcodes are 5 digits, ABCDE
    -- DE: opcode
    -- C: Mode of 1st parameter
    -- B: Mode of 2nd parameter
    -- A: Mode of 3rd paramter
    -- Return true for position, false for intermediate
    -- Will need to change if more modes get added

    local params = math.floor(op / 100)
    local first = (params % 10 == 0)
    params = math.floor(params / 10)
    local second = (params % 10 == 0)
    params = math.floor(params / 10)
    local third = (params % 10 == 0)

    return first, second, third
end

main()
