package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
)

type Coord struct{ x, y, z int }

type Brick struct{ a, b Coord }

func main() {
	bricks := parseInput("input")
	dropBricks(bricks)

	fmt.Printf("Part 1: %d\n", part1(bricks))
	fmt.Printf("Part 2: %d\n", part2(bricks))
}

func part1(bricks []Brick) int {
	supports, supported := getSupportDependencies(bricks)

	total := 0
	for i := range bricks {
		s := true
		for _, j := range supports[i] {
			if len(supported[j]) < 2 {
				s = false
				break
			}
		}
		if s {
			total++
		}
	}
	return total
}

func part2(bricks []Brick) int {
	supports, supported := getSupportDependencies(bricks)

	total := 0
	for i := range bricks {
		var queue []int

		for _, j := range supports[i] {
			if len(supported[j]) == 1 {
				queue = append(queue, j)
			}
		}

		falling := queue
		falling = append(falling, i)

		for len(queue) > 0 {
			j := queue[0]
			queue = queue[1:]

			for _, k := range supports[j] {
				if !contains(falling, k) {
					if isSubset(falling, supported[k]) {
						queue = append(queue, k)
						falling = append(falling, k)
					}
				}
			}
		}
		total += len(falling) - 1
	}
	return total
}

func getSupportDependencies(bricks []Brick) (map[int][]int, map[int][]int) {
	supports := make(map[int][]int)
	supported := make(map[int][]int)

	for i, upper := range bricks {
		for j, lower := range bricks[:i] {
			if checkCollision(lower, upper) &&
				upper.a.z == lower.b.z+1 {

				supports[j] = append(supports[j], i)
				supported[i] = append(supported[i], j)
			}
		}
	}
	return supports, supported
}

func dropBricks(bricks []Brick) {
	sortBricks(bricks)
	for i := range bricks {
		highest := 1
		for j := range bricks[:i] {
			if checkCollision(bricks[i], bricks[j]) {
				highest = max(highest, bricks[j].b.z+1)
			}
		}
		bricks[i].b.z -= bricks[i].a.z - highest
		bricks[i].a.z = highest
	}
	sortBricks(bricks)
}

func checkCollision(lhs, rhs Brick) bool {
	return max(lhs.a.x, rhs.a.x) <= min(lhs.b.x, rhs.b.x) &&
		max(lhs.a.y, rhs.a.y) <= min(lhs.b.y, rhs.b.y)
}

func sortBricks(bricks []Brick) {
	sort.Slice(bricks, func(i, j int) bool {
		return bricks[i].a.z < bricks[j].a.z
	})
}

func parseInput(filename string) []Brick {
	file, _ := os.Open(filename)

	scanner := bufio.NewScanner(file)
	var bricks []Brick

	for scanner.Scan() {
		var b Brick
		fmt.Sscanf(
			scanner.Text(),
			"%d,%d,%d~%d,%d,%d\n",
			&b.a.x, &b.a.y, &b.a.z,
			&b.b.x, &b.b.y, &b.b.z,
		)
		bricks = append(bricks, b)
	}
	return bricks
}
