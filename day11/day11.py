def get_input():
    input = open('day11_input.txt', 'r')
    lines = input.readlines()
    input.close()
    return [[char for char in line.strip()] for line in lines]


def find_first_seat(grid, row, col, row_offset, col_offset, adjacent_only):
    while 0 <= row + row_offset < len(grid) and 0 <= col + col_offset < len(grid[0]):
        seat = grid[row + row_offset][col + col_offset]
        if seat == '#':
            return 1
        elif seat == 'L':
            return 0

        if not adjacent_only:
            row += row_offset
            col += col_offset
        else:
            row = -1
            col = -1

    return 0


def count_occupied(grid, row, col, adjacent_only):
    result = 0
    for i in range(-1, 2):
        for j in range(-1, 2):
            if i != 0 or j != 0:
                result += find_first_seat(grid, row, col, i, j, adjacent_only)

    return result


def count_occupied1(grid, row, col):
    return count_occupied(grid, row, col, True)


def count_occupied2(grid, row, col):
    return count_occupied(grid, row, col, False)


def count_all(grid):
    result = 0
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if grid[row][col] == '#':
                result += 1

    return result


def simulate(count_function, max_occupied):
    grid = get_input()
    done = False

    while not done:
        swaps = []
        done = True
        for row in range(len(grid)):
            for col in range(len(grid[0])):
                if grid[row][col] == 'L':
                    if count_function(grid, row, col) == 0:
                        swaps.append((row, col))
                        done = False
                elif grid[row][col] == '#':
                    if count_function(grid, row, col) >= max_occupied:
                        swaps.append((row, col))
                        done = False

        for swap in swaps:
            row, col = swap[0], swap[1]
            if grid[row][col] == 'L':
                grid[row][col] = '#'
            else:
                grid[row][col] = 'L'

    return count_all(grid)


def part1():
    return simulate(count_occupied1, 4)


def part2():
    return simulate(count_occupied2, 5)


print("Part 1: {}".format(part1()))
print("Part 2: {}".format(part2()))
