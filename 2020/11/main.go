package main

import (
	"github.com/Gerschtli/advent-of-code/lib/go/file"
)

func main() {
}

func parseSeats(filename string) (seats, error) {
	var seatsMap seats

	err := file.ReadFile(filename, func(index int, line string) error {
		var row []status
		for _, c := range line {
			s := statusFree
			if c == '.' {
				s = statusFloor
			}

			row = append(row, s)
		}

		seatsMap = append(seatsMap, row)

		return nil
	})

	if err != nil {
		return nil, err
	}

	return seatsMap, nil
}
