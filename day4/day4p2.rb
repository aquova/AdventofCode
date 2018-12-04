# Advent of Code 2018 - Day 4 Part 2
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

# Find out which guard sleeps the most on a specific minute
longestGuard = nil
longestMinute = nil
longestSleep = 0
sched.each_pair do |k,v|
    minutes = Array.new(60) {|i| 0}
    v.each do |time|
        (time[0]...time[1]).each do |i|
            minutes[i] += 1
        end
    end
    if minutes.max > longestSleep
        longestSleep = minutes.max
        longestMinute = minutes.index(minutes.max)
        longestGuard = k
    end
end

puts longestGuard.to_i * longestMinute
