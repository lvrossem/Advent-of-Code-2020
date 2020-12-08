def replace(instruction):
    if 'nop' in instruction:
        return 'jmp' + instruction[3:]
    elif 'jmp' in instruction:
        return 'nop' + instruction[3:]

def run_program(lines, replaced=0, should_replace=False):
    accumulator, current, looped, visited = 0, 0, False, []

    while not looped and current != len(lines):
        instruction = lines[current]

        if should_replace:
            if current == replaced and ('nop' in instruction or 'jmp' in instruction):
                instruction = replace(instruction)

        if current in visited:
            looped = True
        else:
            visited.append(current)
            if 'nop' in instruction:
                current += 1
            elif 'acc' in instruction:
                accumulator += int(instruction[4:])
                current += 1
            elif 'jmp' in instruction:
                current += int(instruction[4:])

    return accumulator, current == len(lines)

def get_input():
    input = open("day08_input.txt", 'r')
    lines = input.readlines()
    input.close()
    return lines

def part1():
    lines = get_input()

    accumulator, complete = run_program(lines)

    print("Part 1: {}".format(accumulator))

def part2():
    lines, replaced = get_input(), 0

    length = len(lines)

    while replaced < length:
        accumulator, complete = run_program(lines, replaced, True)
        
        replaced += 1

        if complete:
            print("Part 2: {}".format(accumulator))
            replaced = length

part1()
part2()