package main

import (
	"log"

	"github.com/Gerschtli/advent-of-code/lib/go/file"
)

func main() {
	seats, err := readSeats("./files/seats.txt")
	if err != nil {
		log.Fatal(err)
	}

	log.Printf("highest seat id: %d", getHighestId(seats))
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

func getHighestId(seats []seat) int {
	var highestId int
	for _, s := range seats {
		id := s.id()

		if id > highestId {
			highestId = id
		}
	}

	return highestId
}
