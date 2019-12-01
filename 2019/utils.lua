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

return utils
