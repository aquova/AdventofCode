-- Advent of Code 2019 Day 3
package.path = package.path .. ";../?.lua"
local Utils = require("utils")

function main()
    local filename = "input.txt"
    local lines = Utils.load_file(filename)
    local dirs = {}
    for _, line in ipairs(lines) do
        local line_dirs = Utils.split(line, ",")
        table.insert(dirs, line_dirs)
    end

    local coords = {}
    for _, dir in ipairs(dirs) do
        table.insert(coords, gen_coords(dir))
    end

    local matches = find_crosses(coords)
    local smallest = 100000
    for _, k in ipairs(matches) do
        -- local dist = manhattan_dist({10000, 10000}, k)
        -- if dist < smallest then
        --     smallest = dist
        -- end
        if k < smallest then
            smallest = k
        end
    end

    print(smallest)
end

function gen_coords(directions)
    -- Create a large 2D array for our grid
    local coords = {}
    for _ = 1, 20000 do
        table.insert(coords, {})
    end
    -- Doesn't matter where we start, it's all relative
    local pos = {x = 10000, y = 10000}

    local i = 1
    for _, dir in ipairs(directions) do
        local cardinal = string.sub(dir, 1, 1)
        local val = string.sub(dir, 2)
        for _ = 1, val do
            if cardinal == "U" then
                pos.y = pos.y - 1
            elseif cardinal == "D" then
                pos.y = pos.y + 1
            elseif cardinal == "L" then
                pos.x = pos.x - 1
            elseif cardinal == "R" then
                pos.x = pos.x + 1
            end
            -- coords[pos.x][pos.y] = true
            if coords[pos.x][pos.y] == nil then
                coords[pos.x][pos.y] = i
            end

            i = i + 1
        end
    end

    return coords
end

function find_crosses(args)
    local crosses = {}
    local wire1 = args[1]
    local wire2 = args[2]
    for x = 1, 10000 do
        for y = 1, 10000 do
            if wire1[x][y] ~= nil and wire2[x][y] ~= nil then
                local sum = wire1[x][y] + wire2[x][y]
                table.insert(crosses, sum)
            end
        end
    end

    return crosses
end

function manhattan_dist(coord1, coord2)
    return math.abs(coord1[1] - coord2[1]) + math.abs(coord1[2] - coord2[2])
end

main()
