def get_lines():
    input = open('day09_input.txt')
    lines = input.readlines()
    input.close()
    return [int(line) for line in lines]


def is_sum_from_list(list, sum):
    for i in list:
        for j in list:
            if i + j == sum and i != j:
                return True
    return False


def part1():
    lines, current, found = get_lines(), 25, False
    preamble = lines[:25]

    while not found:
        if not is_sum_from_list(preamble, lines[current]):
            found = True
        else:
            current += 1
            preamble = lines[current - 25:current]

    return lines[current]


def part2():
    lines, to_be_found = get_lines(), part1()
    solution, i, found = 0, 0, False

    length = len(lines)

    while i < length and not found:
        j = i
        while j < length and not found:
            if sum(lines[i:j]) == to_be_found:
                solution = min(lines[i:j]) + max(lines[i:j])
                found = True
            j += 1
        i += 1

    return solution

print("Part 1: {}".format(part1()))
print("Part 2: {}".format(part2()))