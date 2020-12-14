import math

directions = ['N', 'E', 'S', 'W']

def get_input():
    input = open('day12_input.txt')
    lines = input.readlines()
    input.close()
    return [line.strip() for line in lines]


def part1():
    lines = get_input()
    start_X, start_Y = 0, 0
    direction = 'E'

    for instruction in lines:
        letter = instruction[0]
        value = int(instruction[1:])

        if letter == 'N':
            start_Y += value
        elif letter == 'E':
            start_X += value
        elif letter == 'S':
            start_Y -= value
        elif letter == 'W':
            start_X -= value
        elif letter == 'R':
            direction = directions[round(directions.index(direction) + value / 90) % len(directions)]
        elif letter == 'L':
            value = 360 - value
            direction = directions[round(directions.index(direction) + value / 90) % len(directions)]
        elif letter == 'F':
            if direction == 'N':
                start_Y += value
            elif direction == 'E':
                start_X += value
            elif direction == 'S':
                start_Y -= value
            elif direction == 'W':
                start_X -= value

    return abs(start_Y) + abs(start_X)


def part2():
    lines = get_input()
    ship_X, ship_Y = 0, 0
    point_X, point_Y = 10, 1

    for instruction in lines:
        letter = instruction[0]
        value = int(instruction[1:])
        
        if letter == 'N':
            point_Y += value
        elif letter == 'E':
            point_X += value
        elif letter == 'S':
            point_Y -= value
        elif letter == 'W':
            point_X -= value
        elif letter == 'F':
            ship_X += value * point_X
            ship_Y += value * point_Y
        else:
            angle = math.radians(-1 * value)
            s, c = math.sin(angle), math.cos(angle)
            if letter == 'L':
                temp_X = round(point_X * c + point_Y * s)
                temp_Y = round(-1 * point_X * s + point_Y * c)
                point_X, point_Y = temp_X, temp_Y
            elif letter == 'R':
                temp_X = round(point_X * c + (-1) * point_Y * s)
                temp_Y = round(point_X * s + point_Y * c)
                point_X, point_Y = temp_X, temp_Y

    return abs(ship_Y) + abs(ship_X)

print("Part 1: {}".format(part1()))
print("Part 2: {}".format(part2()))