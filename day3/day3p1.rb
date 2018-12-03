# Advent of Code 2018 - Day 3 Part 1
# Austin Bricker

input = File.open("input.txt").read
fabric = Array.new(1000) { Array.new(1000) }

# Initializing 2D array to zeroes
for x in 0...1000
    for y in 0...1000
        fabric[x][y] = 0
    end
end

input.each_line do |line|
    words = line.split(' ')
    dimensions = words[3]
    width = dimensions.split('x')[0].to_i
    height = dimensions.split('x')[1].to_i

    margin = words[2][0..-2]
    xMargin = margin.split(',')[0].to_i
    yMargin = margin.split(',')[1].to_i

    for x in xMargin...xMargin+width
        for y in yMargin...yMargin+height
            fabric[y][x] += 1
        end
    end
end

borderTotal = 0
for x in 0...1000
    for y in 0...1000
        if fabric[y][x] > 1
            borderTotal += 1
        end
    end
end

puts borderTotal
