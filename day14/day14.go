package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
	"strings"
)

func main() {
	filename := "sample.txt"
	fmt.Printf("Part 1: %d\n", part1(filename))
	fmt.Printf("Part 2: %d\n", part2(filename))

}

func getInput(filename string) [][]byte {
	readFile, _ := os.Open(filename)

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)
	var input [][]byte

	for fileScanner.Scan() {
		input = append(input, []byte(fileScanner.Text()))
	}
	readFile.Close()
	return input
}

func part1(filename string) int {
	input := getInput(filename)

	return calcLoad(tiltNorth(input))
}

func part2(filename string) int {
	type Image struct {
		i    int
		load int
	}

	input := getInput(filename)
	cache := make(map[string]Image)

	var cycles int

	var inputKey string

	for cycles = 0; ; {
		key := string(bytes.Join(input, nil))
		cache[key] = Image{cycles, calcLoad(input)}
		input = cycle(input)
		cycles += 1
		inputKey = string(bytes.Join(input, nil))

		if _, ok := cache[inputKey]; ok {
			break
		}
	}

	index := cache[inputKey].i + (1_000_000_000-cache[inputKey].i)%(cycles-cache[inputKey].i)
	for _, v := range cache {
		if v.i == index {
			return v.load
		}
	}
	return 0
}

func calcLoad(input [][]byte) int {
	sum := 0
	for i, line := range input {
		sum += (len(input) - i) * strings.Count(string(line), "O")
	}
	return sum
}

func tiltNorth(input [][]byte) [][]byte {
	for c := 0; c < len(string(input[0])); c++ {
		for r := 1; r < len(input); r++ {
			if input[r][c] != byte('O') {
				continue
			}

			n := r - 1
			for n >= 0 && input[n][c] == byte('.') {
				input[n][c] = 'O'
				input[n+1][c] = '.'
				n--
			}
		}
	}
	return input
}

func tiltWest(input [][]byte) [][]byte {
	for r := 0; r < len(input); r++ {
		for c := 1; c < len(string(input[0])); c++ {
			if input[r][c] != byte('O') {
				continue
			}

			n := c - 1
			for n >= 0 && input[r][n] == byte('.') {
				input[r][n] = 'O'
				input[r][n+1] = '.'
				n--
			}
		}
	}
	return input
}

func tiltSouth(input [][]byte) [][]byte {
	for c := 0; c < len(string(input[0])); c++ {
		for r := len(input) - 2; r >= 0; r-- {
			if input[r][c] != byte('O') {
				continue
			}

			n := r + 1
			for n < len(input) && input[n][c] == byte('.') {
				input[n][c] = 'O'
				input[n-1][c] = '.'
				n++
			}
		}
	}
	return input
}

func tiltEast(input [][]byte) [][]byte {
	for r := 0; r < len(input); r++ {
		for c := len(string(input[0])) - 1; c >= 0; c-- {
			if input[r][c] != byte('O') {
				continue
			}

			n := c + 1
			for n < len(string(input[0])) && input[r][n] == byte('.') {
				input[r][n] = 'O'
				input[r][n-1] = '.'
				n++
			}
		}
	}
	return input
}

func cycle(input [][]byte) [][]byte {
	return tiltEast(tiltSouth(tiltWest(tiltNorth(input))))
}
