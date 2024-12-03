with open('input.txt', 'r') as file:
    sum = 0
    for line in file.readlines():
        digits = list(filter(lambda l: ord(l) < 58, line.strip())) # Get only digits (digits have ascii values below 58)
        sum += int(digits[0] + digits[-1])
        
    print(sum)