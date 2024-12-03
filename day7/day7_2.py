order = [ 'J','2','3','4','5','6','7','8','9','T','Q','K','A' ]

def unique_count(s):
    j_count = s.count('J')
    
    # Single edge case, line 648 in input
    if j_count == 5:
        return([j_count], 1)
    
    # Find max letter that is not J
    unique_letters = set(s)
    max_letter = ''
    max_count = 0
    
    for l in unique_letters:
        if l != 'J':
            c = s.count(l) 
            if c > max_count:
                max_count = c
                max_letter = l
                
    new_s = s.replace('J', max_letter)
    
    # Count unique letters as usual (part 1)
    new_unique_letters = set(new_s)
    new_cnt = len(new_unique_letters)
    new_unq = []
    
    for l in new_unique_letters:
        new_unq.append(new_s.count(l))
    
    return (new_unq, new_cnt)

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
    s += data[i][1] * (i+1)

print(s)