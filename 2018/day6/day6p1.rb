# Advent of Code 2018 - Day 6 Part 1
# Austin Bricker

input = File.read("input.txt").split("\n")
# Create a 400x400 grid, large enough to contain points in input with some padding
grid = Array.new(400) { Array.new (400) { |i| -1 } }
totals = Array.new(input.length) { |i| 0 }

# Go through grid and mark initial points with their index
input.each_with_index do |line, i|
    coord = line.split(", ")
    x = coord[0].to_i
    y = coord[1].to_i

    grid[y][x] = i
end

# For each point in grid, find shortest distance to all points in input
(0...grid.length).each do |y|
    (0...grid[y].length).each do |x|
        next if grid[y][x] != -1
        minDistance = 1000000 # A big number
        minIndex = -1
        input.each_with_index do |line, i|
            coord = line.split(", ")
            # Manhattan distance = |dx| + |dy|
            dist = (coord[0].to_i - x).abs + (coord[1].to_i - y).abs
            if dist < minDistance
                minDistance = dist
                minIndex = i
            # Equal distance should be denoted as a tie, in this case -1
            elsif dist == minDistance
                minIndex = -1
            end
        end
        grid[y][x] = minIndex

        if minIndex != -1
            totals[minIndex] += 1
        end
    end
end

# Any value on the edge of the grid is infinite
# Go through sorted order and return first value that isn't on edge
notfound = false
totals.sort.reverse.each do |i|
    next if grid[0].include? totals.index(i)
    next if grid.last.include? totals.index(i)
    (1...grid.length-1).each do |j|
        if totals.index(i) == grid[j][0] or totals.index(i) == grid[j].last
            notfound = true
            break
        end
    end
    if notfound
        notfound = false
        next
    end

    # Add one to include the point itself
    puts i + 1
    break
end

