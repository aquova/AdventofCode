package.path = package.path .. ";../?.lua"
local Utils = require("utils")

local intcode = {}
intcode.__index = intcode

function intcode:new(data)
    local self = setmetatable({}, intcode)
    self.data = Utils.copy_table(data)
    self.pc = 1

    return self
end

function intcode:test()
    print(self.pc)
end

function intcode:parse_opcode(op)
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

function intcode:execute(input)
    while true do
        local op = tonumber(self.data[self.pc])
        local index = op % 100

        -- 99 - Halt
        if index == 99 then
            self.pc = self.pc + 1
            return true
        -- 1 - Addition
        elseif index == 1 then
            local p1, p2, p3 = self:parse_opcode(op)
            -- +1 because Lua is 1-indexed
            local operand_addr1 = self.data[self.pc + 1]
            local operand_addr2 = self.data[self.pc + 2]
            local operand1 = p1 and self.data[operand_addr1 + 1] or operand_addr1
            local operand2 = p2 and self.data[operand_addr2 + 1] or operand_addr2

            local sum = operand1 + operand2
            -- p3 is always true, currently
            local addr = self.data[self.pc + 3]
            self.data[addr + 1] = sum
            self.pc = self.pc + 4
        -- 2 - Multiplication
        elseif index == 2 then
            local p1, p2, p3 = self:parse_opcode(op)

            local operand_addr1 = self.data[self.pc + 1]
            local operand_addr2 = self.data[self.pc + 2]
            local operand1 = p1 and self.data[operand_addr1 + 1] or operand_addr1
            local operand2 = p2 and self.data[operand_addr2 + 1] or operand_addr2

            local prod = operand1 * operand2

            local addr = self.data[self.pc + 3]
            self.data[addr + 1] = prod
            self.pc = self.pc + 4
        -- 3 - Read input
        elseif index == 3 then
            local addr = self.data[self.pc + 1]
            self.data[addr + 1] = tonumber(input)
            self.pc = self.pc + 2
        -- 4 - Output
        elseif index == 4 then
            local p1, _, _ = self:parse_opcode(op)
            local addr = self.data[self.pc + 1]
            local outstr = p1 and self.data[addr + 1] or addr
            -- print("Output: "..outstr)
            self.pc = self.pc + 2
            return outstr
        -- 5 - Jump if not equal
        elseif index == 5 then
            local p1, p2, _ = self:parse_opcode(op)
            local addr1 = self.data[self.pc + 1]
            local addr2 = self.data[self.pc + 2]

            local jmp = p1 and self.data[addr1 + 1] or addr1
            local target = p2 and self.data[addr2 + 1] or addr2
            if tonumber(jmp) ~= 0 then
                self.pc = tonumber(target) + 1
            else
                self.pc = self.pc + 3
            end
        -- 6 - Jump if equal
        elseif index == 6 then
            local p1, p2, _ = self:parse_opcode(op)
            local addr1 = self.data[self.pc + 1]
            local addr2 = self.data[self.pc + 2]

            local jmp = p1 and self.data[addr1 + 1] or addr1
            local target = p2 and self.data[addr2 + 1] or addr2
            if tonumber(jmp) == 0 then
                self.pc = tonumber(target) + 1
            else
                self.pc = self.pc + 3
            end
        -- 7 - Set if less than
        elseif index == 7 then
            local p1, p2, p3 = self:parse_opcode(op)
            local addr1 = self.data[self.pc + 1]
            local addr2 = self.data[self.pc + 2]
            local addr3 = self.data[self.pc + 3]

            local param1 = p1 and self.data[addr1 + 1] or addr1
            local param2 = p2 and self.data[addr2 + 1] or addr2
            self.data[addr3 + 1] = (tonumber(param1) < tonumber(param2)) and 1 or 0
            self.pc = self.pc + 4
        -- 8 - Set if equal
        elseif index == 8 then
            local p1, p2, p3 = self:parse_opcode(op)
            local addr1 = self.data[self.pc + 1]
            local addr2 = self.data[self.pc + 2]
            local addr3 = self.data[self.pc + 3]

            local param1 = p1 and self.data[addr1 + 1] or addr1
            local param2 = p2 and self.data[addr2 + 1] or addr2
            self.data[addr3 + 1] = (tonumber(param1) == tonumber(param2)) and 1 or 0
            self.pc = self.pc + 4
        -- Invalid opcode
        else
            print("Invalid opcode.")
            print(op)
            print(self.pc)
            break
        end
    end
end

return intcode
