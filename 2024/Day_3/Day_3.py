import re
file = open('Input.txt','r')
program_str = str(file.read().split('\n'))
program = re.findall('mul\(([^()]+)\)',program_str)
clean_program = [[int(y) for y in x.split(',')] for x in program]
mult = [x*y for x, y in clean_program]
ans = sum(mult)
print(ans)

#part 2 
test_str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
p2_program = re.findall("mul\(([^()]+)\)",program_str)
# p2_clean_program = [[int(y) for y in x.split(',')] for x in p2_program]
# p2_mult = [x*y for x, y in clean_program]
# p2_ans = sum(mult)
print(p2_program)
