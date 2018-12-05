# Advent of Code Day 5 part 1
# Austin Bricker

input = File.read("input.txt")
letters = input.split("")

restart = true

# Keep looping until there is a loop with no deletions
while restart
    restart = false
    lastLetter = nil
    # For each letter in input, check if previous letter is the same
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

# Remove one at the end, as the \n is still in the array
puts letters.length - 1
