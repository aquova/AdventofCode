package.path = package.path .. ";../?.lua"
local Utils = require("utils")

local intcode = {}
intcode.__index = intcode

function intcode:new(data)
    local self = setmetatable({}, intcode)
    self.data = Utils.copy_table(data)
    self.pc = 0
    self.rel_base = 0

    return self
end

function intcode:read_mem(addr)
    local val = self.data[addr + 1]
    if val == nil then
        val = 0
    end
    return val
end

function intcode:write_mem(addr, val, mode)
    if mode == 2 then
        self.data[self.rel_base + addr + 1] = val
    else
        self.data[addr + 1] = val
    end
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
    local first = params % 10
    params = math.floor(params / 10)
    local second = params % 10
    params = math.floor(params / 10)
    local third = params % 10

    return first, second, third
end

function intcode:parse_mode(addr, mode)
    -- Mode 0 - Position
    if mode == 0 then
        return self:read_mem(addr)
    -- Mode 1 - Immediate
    elseif mode == 1 then
        return addr
    -- Mode 2 - Relative
    elseif mode == 2 then
        return self:read_mem(self.rel_base + addr)
    else
        print("Invalid mode")
    end
end

function intcode:execute(input)
    while true do
        local op = tonumber(self:read_mem(self.pc))
        local index = op % 100

        -- 99 - Halt
        if index == 99 then
            self.pc = self.pc + 1
            return true
        -- 1 - Addition
        elseif index == 1 then
            local p1, p2, p3 = self:parse_opcode(op)
            -- +1 because Lua is 1-indexed
            local operand_addr1 = self:read_mem(self.pc + 1)
            local operand_addr2 = self:read_mem(self.pc + 2)
            local operand1 = self:parse_mode(operand_addr1, p1)
            local operand2 = self:parse_mode(operand_addr2, p2)

            local sum = operand1 + operand2
            local addr = self:read_mem(self.pc + 3)
            self:write_mem(addr, sum, p3)
            self.pc = self.pc + 4
        -- 2 - Multiplication
        elseif index == 2 then
            local p1, p2, p3 = self:parse_opcode(op)

            local operand_addr1 = self:read_mem(self.pc + 1)
            local operand_addr2 = self:read_mem(self.pc + 2)
            local operand1 = self:parse_mode(operand_addr1, p1)
            local operand2 = self:parse_mode(operand_addr2, p2)

            local prod = operand1 * operand2

            local addr = self:read_mem(self.pc + 3)
            self:write_mem(addr, prod, p3)
            self.pc = self.pc + 4
        -- 3 - Read input
        elseif index == 3 then
            local p1, _, _ = self:parse_opcode(op)
            local addr = self:read_mem(self.pc + 1)
            local pos = self:parse_mode(addr, p1)
            self:write_mem(pos, tonumber(input), p1)
            self.pc = self.pc + 2
        -- 4 - Output
        elseif index == 4 then
            local p1, _, _ = self:parse_opcode(op)
            local addr = self:read_mem(self.pc + 1)
            local outstr = self:parse_mode(addr, p1)
            print(outstr)
            self.pc = self.pc + 2
        -- 5 - Jump if not equal
        elseif index == 5 then
            local p1, p2, _ = self:parse_opcode(op)
            local addr1 = self:read_mem(self.pc + 1)
            local addr2 = self:read_mem(self.pc + 2)

            local jmp = self:parse_mode(addr1, p1)
            local target = self:parse_mode(addr2, p2)
            if tonumber(jmp) ~= 0 then
                self.pc = tonumber(target) + 1
            else
                self.pc = self.pc + 3
            end
        -- 6 - Jump if equal
        elseif index == 6 then
            local p1, p2, _ = self:parse_opcode(op)
            local addr1 = self:read_mem(self.pc + 1)
            local addr2 = self:read_mem(self.pc + 2)

            local jmp = self:parse_mode(addr1, p1)
            local target = self:parse_mode(addr2, p2)
            if tonumber(jmp) == 0 then
                self.pc = tonumber(target) + 1
            else
                self.pc = self.pc + 3
            end
        -- 7 - Set if less than
        elseif index == 7 then
            local p1, p2, p3 = self:parse_opcode(op)
            local addr1 = self:read_mem(self.pc + 1)
            local addr2 = self:read_mem(self.pc + 2)
            local addr3 = self:read_mem(self.pc + 3)

            local param1 = self:parse_mode(addr1, p1)
            local param2 = self:parse_mode(addr2, p2)
            local val = (tonumber(param1) < tonumber(param2)) and 1 or 0
            self:write_mem(addr3, val, p3)

            self.pc = self.pc + 4
        -- 8 - Set if equal
        elseif index == 8 then
            local p1, p2, p3 = self:parse_opcode(op)
            local addr1 = self:read_mem(self.pc + 1)
            local addr2 = self:read_mem(self.pc + 2)
            local addr3 = self:read_mem(self.pc + 3)

            local param1 = self:parse_mode(addr1, p1)
            local param2 = self:parse_mode(addr2, p2)
            local val = (tonumber(param1) == tonumber(param2)) and 1 or 0
            self:write_mem(addr3, val, p3)
            self.pc = self.pc + 4
        -- 9 - Set Relative Base
        elseif index == 9 then
            local p1, _, _ = self:parse_opcode(op)
            local addr = self:read_mem(self.pc + 1)
            self.rel_base = self.rel_base + self:parse_mode(addr, p1)
            -- print("Relative Base: "..self.rel_base)
            self.pc = self.pc + 2
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
