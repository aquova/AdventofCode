-- Advent of Code - Day 6

package.path = package.path .. ";../?.lua"
local Utils = require("utils")

function main()
    local filename = "input.txt"
    local f = Utils.load_file(filename)
    local orbits = {}
    for _, line in ipairs(f) do
        local orbit = Utils.split(line, ")")
        orbits[orbit[2]] = orbit[1]
    end

    -- print(count_orbits(orbits))
    print(count_transfers(orbits))
end

function count_orbits(orbits)
    local count = 0
    for k, v in pairs(orbits) do
        local planet = k
        while planet ~= "COM" do
            planet = orbits[planet]
            count = count + 1
        end
    end

    return count
end

function count_transfers(orbits)
    -- First, create table of transfers from YOU to COM
    local you_transfers = {}
    local planet = "YOU"
    while planet ~= "COM" do
        planet = orbits[planet]
        table.insert(you_transfers, planet)
    end

    -- Do the same for SAN
    local san_transfers = {}
    local planet = "SAN"
    while planet ~= "COM" do
        planet = orbits[planet]
        table.insert(san_transfers, planet)
    end

    -- Find earliest common planet
    for i, k in ipairs(you_transfers) do
        if Utils.in_tbl(k, san_transfers) then
            return i + Utils.index_of(k, san_transfers) - 2
        end
    end

    print("Something went wrong")
    return -1
end

main()
