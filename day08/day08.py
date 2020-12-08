def part1():
    input = open("day08_input.txt", 'r')

    lines = input.readlines()

    accumulator, current, looped, visited = 0, 0, False, []

    while not looped:
        instruction = lines[current]

        if current in visited:
            looped = True
        else:
            visited.append(current)
            if 'nop' in instruction:
                current += 1
            elif 'acc' in instruction:
                mul = 1
                if instruction[4] == '-':
                    mul = -1
                
                number = int(instruction[5:])

                accumulator += number * mul
                current += 1
            elif 'jmp' in instruction:
                mul = 1
                if '-' in instruction:
                    mul = -1
                
                number = int(instruction[5:])

                current += number * mul

    print("Part 1: {}".format(accumulator))

    input.close()

def replace(instruction):
    if 'nop' in instruction:
        return 'jmp' + instruction[3:]
    elif 'jmp' in instruction:
        return 'nop' + instruction[3:]

def part2():
    input = open("day08_input.txt", 'r')

    lines = input.readlines()

    replaced = 0

    while replaced < len(lines):
        accumulator, current, looped, visited = 0, 0, False, []

        while not looped and current != len(lines):
            instruction = lines[current]

            if current == replaced and ('nop' in instruction or 'jmp' in instruction):
                instruction = replace(instruction)

            if current in visited:
                looped = True
            else:
                visited.append(current)
                if 'nop' in instruction:
                    current += 1
                elif 'acc' in instruction:
                    mul = 1
                    if instruction[4] == '-':
                        mul = -1
                    
                    number = int(instruction[5:])

                    accumulator += number * mul
                    current += 1
                elif 'jmp' in instruction:
                    mul = 1
                    if '-' in instruction:
                        mul = -1
                    
                    number = int(instruction[5:])

                    current += number * mul
        replaced += 1

        if current == len(lines):
            print("Part 2: " + str(accumulator))
            replaced = len(lines)

part1()
part2()

