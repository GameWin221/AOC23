color_counts = {'red' : 12, 'green' : 13, 'blue' : 14}

sum = 0

def is_possible(line):
    turns = line.strip().split(':')[1].strip().split(';')
        
    for turn in turns:
        colors = turn.split(',')
        
        for color in colors:
            color_data = color.strip().split() # [color_count, color_name]
            
            if int(color_data[0]) > color_counts[color_data[1]]:
                return False
            
    return True

with open('input.txt', 'r') as file:
    for idx, line in enumerate(file.readlines()):
        if is_possible(line):
            sum += idx + 1
              
print(sum)