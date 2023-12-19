package main

import (
	"testing"
)

func Test_part1_sample(t *testing.T) {
	result := part1("sample.txt")
	actual := 136

	if result != actual {
		t.Errorf("part1(\"sample.txt\") returned %d, expected: %d", result, actual)
	}
}

func Test_part1_input(t *testing.T) {
	result := part1("input")
	actual := 113486

	if result != actual {
		t.Errorf("part1(\"input\") returned %d, expected: %d", result, actual)
	}
}

func Test_part2_sample(t *testing.T) {
	result := part2("sample.txt")
	actual := 64

	if result != actual {
		t.Errorf("part2(\"sample.txt\") returned %d, expected: %d", result, actual)
	}
}

func Test_part2_input(t *testing.T) {
	result := part2("input")
	actual := 104409

	if result != actual {
		t.Errorf("part2(\"input\") returned %d, expected: %d", result, actual)
	}
}
