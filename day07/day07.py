bags = dict()

with open("day07_input.txt") as input:
    indices = []
    lines = input.readlines()
    for line in lines:
        split = line.split(' bags contain')
        color = split[0]

        bags[color] = dict()
        bags[color]['contains'] = []
        bags[color]['contained_by'] = []

    for line in lines:
        split = line.split(' bags contain')
        color, contains = split[0], split[1]

        if 'no other' not in line:
            for content in contains.rsplit(' ', 1)[0].split(', '):
                content_split = content.strip().split(' ')
                bags[color]['amount'] = int(content_split[0])

                contain_color = content_split[1] + ' ' + content_split[2]
                bags[color]['contains'].append((contain_color, int(content_split[0])))

                bags[contain_color]['contained_by'].append(color)

visited = []
next = ['shiny gold']
found_all = False

while not found_all:
    temp = []
    done = True
    for color in next:
        color_contained_by = bags[color]['contained_by']
        for upper_color in color_contained_by:
            temp.append(upper_color)
            if upper_color not in visited:
                visited.append(upper_color)
                done = False

    found_all = done
    next = temp.copy()

print("Part 1: {}".format(len(visited)))


def get_content_recursive(color, amount):
    if len(bags[color]['contains']) == 0:
        return 0
    else:
        result = 0
        for contains_color in bags[color]['contains']:
            result += contains_color[1] + contains_color[1] * get_content_recursive(contains_color[0], contains_color[1])
        return result

print('Part 2: {}'.format(str(get_content_recursive('shiny gold', 1))))