# Advent of Code - Day 8 Part 1
# Austin Bricker

input = File.read("test.txt")
vals = input.split(" ").map(&:to_i)
p vals

total = 0

def getValues(i)

end

while i < vals.length
    i_numChildren = vals[i]
    i += 1
    i_numMeta = vals[i]
    i += i_numChildren + 1
    (0...i_numMeta).each do |j|
        total += vals[i]
        i += 1
    end
end

puts total
