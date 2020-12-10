package main

import (
	"strconv"

	"github.com/Gerschtli/advent-of-code/lib/go/file"
)

func main() {
}

func parseFile(filename string) ([]int, error) {
	var numbers []int
	err := file.ReadFile(filename, func(index int, line string) error {
		num, err := strconv.Atoi(line)
		if err != nil {
			return err
		}

		numbers = append(numbers, num)
		return nil
	})

	if err != nil {
		return nil, err
	}

	return numbers, nil
}
