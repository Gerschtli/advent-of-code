package main

import "github.com/Gerschtli/advent-of-code/lib/go/file"

func main() {
}

func readSeats(filename string) ([]seat, error) {
	var seats []seat

	err := file.ReadFile(filename, func(index int, line string) error {
		s, err := buildSeatByCode(line)
		if err != nil {
			return err
		}

		seats = append(seats, s)
		return nil
	})

	if err != nil {
		return nil, err
	}

	return seats, nil
}
