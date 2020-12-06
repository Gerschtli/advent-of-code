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
	log.Printf("free seat id: %d", getFreeSeat(seats)[0].id())
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

func getFreeSeat(seats []seat) []seat {
	p := buildPlane(128, 8)
	for _, s := range seats {
		p.markSeatTaken(&s)
	}

	p.shrinkPlaneSize()

	// "Your seat wasn't at the very front or back"
	p.markFrontAndBackRowTaken()

	return p.getFreeSeats()
}
