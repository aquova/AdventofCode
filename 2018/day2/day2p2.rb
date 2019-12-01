# Advent of Code 2018 - Day 2 Part 2
# Austin Bricker

# Switching to Ruby because I hate Rust

text = IO.readlines("input.txt")
lineNum = text.length

for i in 0...lineNum
    for j in i+1...lineNum
        line1 = text[i]
        line2 = text[j]
        differences = 0

        for k in 0..line1.length
            if line1[k] != line2[k]
                differences += 1
            end
        end

        if differences == 1
            # You do have to eyeball to find the correct answer, but oh well
            puts line1
            puts line2
            return
        end
    end
end
