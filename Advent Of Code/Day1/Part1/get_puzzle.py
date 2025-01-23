a = open("puzzles.txt")
text = a.read()
arr = text.split("\n")
arr1 = []
arr2 = []
for i in range(len(arr)):
    arr1.append(int(arr[i].split(" ")[0]))
    arr2.append(int(arr[i].split(" ")[1]))
print(arr2)