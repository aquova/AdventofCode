# Advent of Code 2018 - Day 4 Part 1
# Austin Bricker

input = IO.readlines("input.txt").sort

guard = nil
asleep = nil
sched = Hash.new {|k,v| k[v]=[]}

# Generate a hashmap of the sleep times for each guard
input.each do |line|
    time = line[15..16].to_i
    words = line.split(' ')
    if words[2] == "Guard"
        guard = words[3][1..-1]
        asleep = nil
    elsif words[2] == "wakes"
        if asleep != nil and guard != nil
            sched[guard] << [asleep, time]
        end
    else
        asleep = time
    end
end

# Find out which guard sleeps the longest
longestGuard = nil
longestSleep = 0
sched.each_pair do |k,v|
    total = 0
    v.each do |time|
        total += time[1] - time[0]
    end
    if total > longestSleep
        longestGuard = k
        longestSleep = total
    end
end

# Now find what minute they're asleep for the most.
# Create a 60 element array initialized to 0, just count up
minutes = Array.new(60) {|i| 0}
sched[longestGuard].each do |time|
    (time[0]...time[1]).each do |i|
        minutes[i] += 1
    end
end

puts minutes.index(minutes.max) * longestGuard.to_i
