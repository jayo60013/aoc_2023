import pdb

DIRECTIONS = {
    "up": [0, -1],
    "down": [0, 1],
    "left": [-1, 0],
    "right": [1, 0],
}
WIDTH = 0
HEIGHT = 0
GRID = []


def findStart(grid):
    for y, line in enumerate(grid):
        x = line.find("S")
        if x != -1:
            return [x, y]


def checkDirection(dir, pipe) -> bool:
    if dir == "up":
        return pipe == "|" or pipe == "F" or pipe == "7"
    elif dir == "down":
        return pipe == "|" or pipe == "L" or pipe == "J"
    elif dir == "left":
        return pipe == "-" or pipe == "F" or pipe == "L"
    elif dir == "right":
        return pipe == "-" or pipe == "7" or pipe == "J"
    else:
        return False


def checkBounds(x, y) -> bool:
    return 0 <= x < WIDTH and 0 <= y < HEIGHT


def startPipe(positions, lastVisited):
    newPositions = []

    for pos, prev in zip(positions, lastVisited):
        for dir, delta in DIRECTIONS.items():
            x = pos[0] + delta[0]
            y = pos[1] + delta[1]
            if checkBounds(x, y) and checkDirection(dir, GRID[y][x]):
                if [x, y] != prev:
                    newPositions.append([x, y])

    return newPositions, positions


def stepPipe(positions, lastVisited):
    newPositions = []

    for pos, prev in zip(positions, lastVisited):
        for dir, delta in DIRECTIONS.items():
            x = pos[0] + delta[0]
            y = pos[1] + delta[1]
            if checkBounds(x, y) and checkDirection(dir, GRID[y][x]):
                if [x, y] != prev:
                    newPositions.append([x, y])
                    break

    return newPositions, positions


def part1():
    positions = []
    lastVisited = []
    positions.append(findStart(GRID))
    lastVisited = [positions, positions]

    positions, lastVisited = startPipe(positions, lastVisited)
    lastVisited = [positions, positions]

    # pdb.set_trace()
    count = 0
    while positions[0] != positions[1]:
        positions, lastVisited = stepPipe(positions, lastVisited)
        print(positions)
        count += 1

    print(count)


with open("sample.txt") as file:
    for line in file:
        GRID.append(line)
WIDTH = len(GRID[0])
HEIGHT = len(GRID)

part1()
# print(f"Part 1: {part1(lines)}")
# print(f"Part 2: {part2(lines)}")
file.close()
