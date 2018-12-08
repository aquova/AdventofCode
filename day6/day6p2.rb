# Advent of Code 2018 - Day 6 Part 2
# Austin Bricker

input = File.read("input.txt").split("\n")
# Create a 400x400 grid, large enough to contain points in input with some padding
grid = Array.new(400) { Array.new (400) { |i| "." } }
grandTotal = 0
visualize = true

if visualize
    # Go through grid and mark initial points with their index
    input.each_with_index do |line, i|
        coord = line.split(", ")
        x = coord[0].to_i
        y = coord[1].to_i

        grid[y][x] = "X"
    end
end

# For each point in grid, sum up distances to all coordinate in input
(0...grid.length).each do |y|
    (0...grid[y].length).each do |x|
        subtotal = 0
        input.each_with_index do |line, i|
            coord = line.split(", ")
            # Manhattan distance = |dx| + |dy|
            dist = (coord[0].to_i - x).abs + (coord[1].to_i - y).abs
            subtotal += dist
        end
        if subtotal < 10000
            grandTotal += 1
            grid[y][x] = "#" if visualize
        end
    end
end

if visualize
    grid.each do |line|
        puts line.join("")
    end
end

puts grandTotal
