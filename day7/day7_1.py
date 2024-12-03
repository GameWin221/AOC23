order = [ '2','3','4','5','6','7','8','9','T','J','Q','K','A' ]

def unique_count(s) :
    unique_letters = set(s)
    cnt = len(unique_letters)
    unq = []
    
    for l in unique_letters:
        unq.append(s.count(l))
    
    return (unq, cnt)

def level_count(s):
    (unique, count) = unique_count(s)
    
    if count == 1: # Five of a kind
        return 6
    elif count == 2: 
        if 4 in unique: # Four of a kind
            return 5
        else:           # Full house
            return 4
    elif count == 3:
        if 3 in unique: # Three of a kind
            return 3
        else:           # Two pair 
            return 2 
    elif count == 4: # One pair 
        return 1
    else: # High card
        return 0

def a_gt_b(a, b):
    lvl_a = level_count(a)
    lvl_b = level_count(b)

    if lvl_a != lvl_b:
        return lvl_a > lvl_b

    for l in range(0, 5):
        if a[l] != b[l]:
            return order.index(a[l]) > order.index(b[l])

data = []

# Read and convert codes
with open('input.txt') as file:
    for line in file.readlines():
        code = line[0:5]
        bid = int(line[6:-1])

        data.append((code, bid))

# Sort codes with bubble sort
for i in range(len(data)-1):
    for j in range(0, len(data)-i-1):
        if a_gt_b(data[j][0], data[j+1][0]):
            data[j], data[j+1] = data[j+1], data[j]

s = 0       
for i in range(0, len(data)):
    print(f"{data[i][1]} * {(i+1)}")
    s += data[i][1] * (i+1)
    
print(s)