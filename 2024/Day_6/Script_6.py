file = open("puzzle.txt",'r')
str_initial_map = file.read()
list_initial_map = str_initial_map.split("\n")
pl_x = [[idx, str] for idx, str in enumerate(list_initial_map) if "^" in str]
pl_y_idx = int(pl_x[0][0])
pl_x = pl_x[0][1]
pl_x_idx = int(pl_x.find("^"))
if list_initial_map[pl_y_idx][pl_x_idx]!="#":
    map = list_initial_map[pl_y_idx-1][pl_x_idx].replace(".","^",1)
print(map)
