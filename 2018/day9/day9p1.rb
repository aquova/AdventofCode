# Advent of Code - Day 9 Part 1
# Austin Bricker

# Don't even bother reading in from input, just hard code it, it's simple enough

players = 404
num_marbles = 71852
marbles = [0]
curr_index = 0
curr_player = 0

(1...num_marbles).each do |x|
    next_index = (curr_index + 2) % marbles.length
    marbles[next_index] = x
    curr_index = next_index
    curr_player = (curr_player + 1) % players
end
