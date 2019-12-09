-- Advent of Code 2019 Day 8
package.path = package.path .. ";../?.lua"
local Utils = require("utils")

function main()
    local PIC = {WIDTH = 25, HEIGHT = 6}
    local LAYER_SIZE = PIC.WIDTH * PIC.HEIGHT
    local FILENAME = "input.txt"
    local f = Utils.load_file(FILENAME)[1]
    local lf = layer_factory()

    local layers = {}
    for i = 1, #f, LAYER_SIZE do
        local data = f:sub(i, i + LAYER_SIZE - 1)
        local new_layer = lf:new(data)
        table.insert(layers, new_layer)
    end

    local fewest_zero_index = 0
    local fewest_zeros = LAYER_SIZE + 1
    for i, layer in ipairs(layers) do
        local count = layer:count_digits(0)
        if count < fewest_zeros then
            fewest_zeros = count
            fewest_zero_index = i
        end
    end

    print(layers[fewest_zero_index]:count_digits(1) * layers[fewest_zero_index]:count_digits(2))
end

function layer_factory()
    local layer = {}
    layer.__index = layer

    function layer:new(data)
        local self = setmetatable({}, layer)
        self.data = data

        return self
    end

    function layer:count_digits(digit)
        local count = 0
        for i = 1, #self.data do
            if tonumber(self.data:sub(i, i)) == digit then
                count = count + 1
            end
        end

        return count
    end

    return layer
end

main()
