file = open("Puzzle_Input.txt", 'r')
reports_str = file.read()
reports_ugly = reports_str.splitlines()
reports = [[int(y) for y in x.split(' ')] for x in reports_ugly]
deltas = [[report[num] - report[num + 1] for num in range(len(report) - 1)] for report in reports]
not_safe_deltas = []
for delta in deltas:
    for num in delta:
        if abs(num) > 3 or abs(num) < 1:
            not_safe_deltas.append(delta)
            break
safe_deltas = [delta for delta in deltas if delta not in not_safe_deltas]
not_safe_deltas = []
for delta in safe_deltas:
    for num in range(len(delta) - 1):
        if delta[num] * delta[num + 1] < 0:
            not_safe_deltas.append(delta)
            break
safe_deltas = [delta for delta in safe_deltas if delta not in not_safe_deltas]
print(len(safe_deltas))

# Part 2
sorted_deltas = [sorted(delta, reverse=True) for delta in deltas]
safe_deltas = []
for delta in sorted_deltas:
    if 0 < abs(delta[1]) < 4 and delta[0] * delta[-2] > 0 and 0 < abs(delta[-1]) < 4 :
        safe_deltas.append(delta)
print(len(safe_deltas))
