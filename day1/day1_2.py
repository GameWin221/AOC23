delimiters = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']
word_to_num = { 'on': 1, 'tw' : 2, 'th' : 3, 'fo' : 4, 'fi' : 5, 'si' : 6, 'se' : 7, 'ei' : 8, 'ni' : 9 }

import time

with open('input.txt', 'r') as file:
    sum = 0
    
    for line in file.readlines():
        left_idx = 100000
        right_idx = -1

        # Find the leftmost and the rightmost numbers (numerically or verbally)
        for delim in (d for d in delimiters if d in line): 
            left_idx = min(left_idx, line.index(delim))
            right_idx = max(right_idx, line.rindex(delim))
                
        left_digit = 0
        right_digit = 0

        if line[left_idx].isdigit():
            left_digit = int(line[left_idx])
        else:
            left_digit = word_to_num[line[left_idx:left_idx+2]]

        if line[right_idx].isdigit():
            right_digit = int(line[right_idx])
        else:
            right_digit = word_to_num[line[right_idx:right_idx+2]]

        sum += left_digit * 10 + right_digit       
        
    print(sum)