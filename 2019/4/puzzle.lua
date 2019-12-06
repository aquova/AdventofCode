-- Advent of Code 2019 - Day 4
function main()
    local min = 197487
    local max = 673251
    local count = 0
    for i = min, max do
        if check_doubles(i) and check_increasing(i) then
            count = count + 1
        end
    end

    print(count)
end

function check_doubles(num)
    local str = tostring(num)
    for i = 0, 9 do
        local double = string.rep(i, 2)
        if string.find(str, double) ~= nil then
            local triple = string.rep(i, 3)
            if string.find(str, triple) == nil then
                return true
            end
        end
    end

    return false
end

function check_increasing(num)
    local str = tostring(num)
    local len = str:len()
    for i = 1, len - 1 do
        if str:sub(i, i) > str:sub(i + 1, i + 1) then
            return false
        end
    end

    return true
end

main()
