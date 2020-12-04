-- Some useful utility functions,
-- so I don't have to keep rewriting them

local utils = {}
utils.__index = utils

function utils.load_file(filename)
    local f = assert(io.open(filename, "rb"))
    local lines = {}
    for line in io.lines(filename) do
        table.insert(lines, line)
    end

    return lines
end

function utils.print_table(tbl, show_index)
    for i, k in ipairs(tbl) do
        local str = ""
        if show_index then
            str = "Line "..i..": "
        end

        str = str..k
        print(str)
    end
end

-- From here: https://stackoverflow.com/questions/1426954/split-string-in-lua
function utils.split(str, delim)
    if delim == nil then
        return str
    end

    local words = {}
    for w in str:gmatch("([^"..delim.."]+)") do
        table.insert(words, w)
    end

    return words
end

function utils.copy_table(tbl)
    local copy = {}
    for _, k in ipairs(tbl) do
        table.insert(copy, k)
    end
    return copy
end

function utils.in_tbl(elem, tbl)
    for _, k in ipairs(tbl) do
        if elem == k then
            return true
        end
    end

    return false
end

function utils.index_of(elem, tbl)
    for i, k in ipairs(tbl) do
        if elem == k then
            return i
        end
    end

    return nil
end

function utils.gen_2d_array(width, height, default_val)
    local grid = {}
    for i = 1, height do
        local row = {}
        for j = 1, width do
            table.insert(row, default_val)
        end
        table.insert(grid, row)
    end

    return grid
end

return utils
