sum = 0

with open('input.txt', 'r') as file:
    for line in file.readlines():
        turns = line.strip().split(':')[1].strip().split(';')
        
        max_colors = {'red': 0, 'green': 0, 'blue': 0}
        
        for turn in turns:
            colors = turn.split(',')

            for color in colors:
                color_data = color.strip().split()  # [color_count, color_name]

                max_colors[color_data[1]] = max(max_colors[color_data[1]], int(color_data[0]))
            
        sum += max_colors['red'] * max_colors['green'] * max_colors['blue']
              
print(sum)