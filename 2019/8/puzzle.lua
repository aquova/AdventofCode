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
    local index = 1
    for i = 1, #f, LAYER_SIZE do
        local data = f:sub(i, i + LAYER_SIZE - 1)
        local new_layer = lf:new(data)
        layers[index] = new_layer
        index = index + 1
    end

    -- Part 1:
    -- local fewest_zero_index = 0
    -- local fewest_zeros = LAYER_SIZE + 1
    -- for i, layer in ipairs(layers) do
    --     local count = layer:count_digits(0)
    --     if count < fewest_zeros then
    --         fewest_zeros = count
    --         fewest_zero_index = i
    --     end
    -- end
    -- print(layers[fewest_zero_index]:count_digits(1) * layers[fewest_zero_index]:count_digits(2))

    -- Part 2:
    local image = {}
    for i = 1, LAYER_SIZE do
        table.insert(image, 2)
    end

    for index = 1, #layers do
        for i = 1, LAYER_SIZE do
            if image[i] == 2 then
                image[i] = layers[index]:get_pixel(i)
            end
        end
    end

    print_image(image, PIC.HEIGHT, PIC.WIDTH)
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
            if self:get_pixel(i) == digit then
                count = count + 1
            end
        end

        return count
    end

    function layer:get_pixel(offset)
        return tonumber(self.data:sub(offset, offset))
    end

    return layer
end

function print_image(image, height, width)
    local pixels = { [0] = "⬜ ", "⬛ ", "X "}
    for row = 0, height - 1 do
        text = ""
        for col = 0, width - 1 do
            text = text .. pixels[image[col + row * width + 1]]
        end
        print(text)
    end
end

main()
