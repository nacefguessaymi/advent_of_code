# Part 1
puzzle = open("Puzzle_input.txt","r")
list = puzzle.read()
new_list = list.split("\n")
left = [int(x.split("   ") [0])for x in new_list]
right = [int(x.split("   ") [1])for x in new_list]
left.sort()
right.sort()
tot_dist = 0
for num in range(len(left)):
    dist = abs(left[num]-right[num])
    tot_dist = tot_dist + dist
print(tot_dist)

# Part 2
score = 0 
for num in range(len(left)):
    count = right.count(left[num])
    score = score + (count * left[num])
print(score)