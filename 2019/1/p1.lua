-- Advent of Code Puzzle 1 - Part 1
package.path = package.path .. ";../?.lua"
local Utils = require("utils")

function main()
    local filename = "input1.txt"
    local lines = Utils.load_file(filename)

    -- For Part 1:
    -- local fuels = calc_fuel(lines)

    -- For Part 2:
    local fuels = calc_fuel_compound(lines)

    print("Sum: "..sum_fuels(fuels))
end

function calc_fuel(vals)
    local fuel = {}
    for _, k in ipairs(vals) do
        local out = math.floor(k / 3) - 2
        table.insert(fuel, out)
    end

    return fuel
end

function calc_fuel_compound(vals)
    local fuels = {}
    for _, k in ipairs(vals) do
        local fuel = { k }

        while true do
            local next = math.floor(fuel[#fuel] / 3) - 2

            if next <= 0 then
                break
            else
                table.insert(fuel, next)
            end
        end

        local sum = sum_fuels(fuel) - k
        table.insert(fuels, sum)
    end

    return fuels
end

function sum_fuels(vals)
    local sum = 0
    for _, k in ipairs(vals) do
        sum = sum + k
    end

    return sum
end

main()
