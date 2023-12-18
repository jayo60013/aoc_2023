package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	readFile, _ := os.Open("sample.txt")

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)
	var input [][]byte

	for fileScanner.Scan() {
		input = append(input, []byte(fileScanner.Text()))
	}

	for c := 0; c < len(input); c++ {
		for r := 0; r < len(input[0]); r++ {
			fmt.Println("Suck your mum")
		}
	}

	readFile.Close()
}
