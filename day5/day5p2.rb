# Advent of Code Day 5 Part 2
# Austin Bricker

# This runs super poorly

input = File.read("input.txt")
# Removes new line at the end
readLetters = input.split("")[0...-1]

# Create a tally for each letter
alphabet = Hash.new()
('a'..'z').each {|letter| alphabet[letter]=0}

alphabet.keys.each do |x|
    # First, create a version with the current letter missing
    letters = []
    readLetters.each do |l|
        if l.upcase != x.upcase
            letters.push(l)
        end
    end

    # Same procedure as in part 1
    restart = true
    while restart
        restart = false
        lastLetter = nil
        letters.each_with_index do |letter, i|
            if lastLetter == nil
                lastLetter = letter
            else
                if (lastLetter != letter) and (lastLetter.upcase == letter.upcase)
                    restart = true
                    letters.delete_at(i)
                    letters.delete_at(i-1)
                    lastLetter = nil
                else
                    lastLetter = letter
                end
            end
        end
    end

    alphabet[x] = letters.length
    puts "Finished with #{x}"
end

min, max = alphabet.values.minmax
puts min
