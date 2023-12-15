from collections import deque


def findStart(grid):
    for y, line in enumerate(grid):
        x = line.find("S")
        if x != -1:
            return (y, x)


def part1(grid):
    start = findStart(grid)
    visited = {start}
    toVisit = deque([start])

    while len(toVisit) != 0:
        row, col = toVisit.popleft()
        sym = grid[row][col]

        if (
            row > 0
            and sym in "S|JL"
            and grid[row - 1][col] in "|7F"
            and (row - 1, col) not in visited
        ):
            visited.add((row - 1, col))
            toVisit.append((row - 1, col))

        if (
            row < len(grid) - 1
            and sym in "S|7F"
            and grid[row + 1][col] in "|JL"
            and (row + 1, col) not in visited
        ):
            visited.add((row + 1, col))
            toVisit.append((row + 1, col))

        if (
            col > 0
            and sym in "S-J7"
            and grid[row][col - 1] in "-LF"
            and (row, col - 1) not in visited
        ):
            visited.add((row, col - 1))
            toVisit.append((row, col - 1))

        if (
            col < len(grid[row]) - 1
            and sym in "S-LF"
            and grid[row][col + 1] in "-J7"
            and (row, col + 1) not in visited
        ):
            visited.add((row, col + 1))
            toVisit.append((row, col + 1))

    return len(visited) // 2


def removeNonLoopPipes(grid, visited):
    new_grid = []
    for r, row in enumerate(grid):
        new_row = ""
        for c, sym in enumerate(row):
            if (r, c) in visited:
                new_row += sym
            else:
                new_row += "."
        new_grid.append(new_row)
    return new_grid


def part2(grid):
    start = findStart(grid)
    visited = {(start)}
    toVisit = deque([(start)])

    start_symbols = {"|", "-", "J", "L", "7", "F"}

    while toVisit:
        row, col = toVisit.popleft()
        sym = grid[row][col]

        if (
            row > 0
            and sym in "S|JL"
            and grid[row - 1][col] in "|7F"
            and (row - 1, col) not in visited
        ):
            visited.add((row - 1, col))
            toVisit.append((row - 1, col))
            if sym == "S":
                start_symbols &= {"|", "J", "L"}

        if (
            row < len(grid) - 1
            and sym in "S|7F"
            and grid[row + 1][col] in "|JL"
            and (row + 1, col) not in visited
        ):
            visited.add((row + 1, col))
            toVisit.append((row + 1, col))
            if sym == "S":
                start_symbols &= {"|", "7", "F"}

        if (
            col > 0
            and sym in "S-J7"
            and grid[row][col - 1] in "-LF"
            and (row, col - 1) not in visited
        ):
            visited.add((row, col - 1))
            toVisit.append((row, col - 1))
            if sym == "S":
                start_symbols &= {"-", "J", "7"}

        if (
            col < len(grid[row]) - 1
            and sym in "S-LF"
            and grid[row][col + 1] in "-J7"
            and (row, col + 1) not in visited
        ):
            visited.add((row, col + 1))
            toVisit.append((row, col + 1))
            if sym == "S":
                start_symbols &= {"-", "L", "F"}

    if len(start_symbols) != 1:
        print("ERR: Could not get start symbol")
        exit(0)
    (S,) = start_symbols
    grid = [row.replace("S", S) for row in grid]
    grid = removeNonLoopPipes(grid, visited)

    outside = set()

    for r, row in enumerate(grid):
        within = False
        up = None
        for c, sym in enumerate(row):
            if sym == "|":
                within = not within
            elif sym in "LF":
                up = sym == "L"
            elif sym in "7J":
                if sym != ("J" if up else "7"):
                    within = not within
                up = None
            if not within:
                outside.add((r, c))

    print(len(grid) * len(grid[0]) - len(outside | visited))


grid = open("input").read().strip().splitlines()
# print(f"Part 1: {part1(grid)}")
part2(grid)
# print(f"Part 2: {part2(lines)}")
