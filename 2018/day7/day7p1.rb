# Advent of Code 2018 - Day 7 Part 1
# Austin Bricker

input = File.read("input.txt").split("\n")
alphabet = ("A".."Z").to_a

input.each do |line|
    before = line.split(" ")[1]
    after = line.split(" ")[7]
    if alphabet.index(after) < alphabet.index(before)
        alphabet[alphabet.index(before)], alphabet[alphabet.index(after)] = alphabet[alphabet.index(after)], alphabet[alphabet.index(before)]
    end
end

puts alphabet.join("")
