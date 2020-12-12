package main

import (
	"log"
	"reflect"

	"github.com/Gerschtli/advent-of-code/lib/go/file"
)

func main() {
	seatsMap, err := parseSeats("./files/seats.txt")
	if err != nil {
		log.Fatal(err)
	}

	currentSeats, newSeats := seatsMap, seats{}
	for {
		newSeats = currentSeats.runRound(currentSeats.countOccupied, 4)

		if reflect.DeepEqual(currentSeats, newSeats) {
			log.Printf("%d occupied seats\n", newSeats.countAllOccupied())
			break
		}

		currentSeats = newSeats
	}

	currentSeats, newSeats = seatsMap, seats{}
	for {
		newSeats = currentSeats.runRound(currentSeats.countOccupiedInSight, 5)

		if reflect.DeepEqual(currentSeats, newSeats) {
			log.Printf("%d occupied seats\n", newSeats.countAllOccupied())
			break
		}

		currentSeats = newSeats
	}
}

func parseSeats(filename string) (seats, error) {
	var seatsMap seats

	err := file.ReadFile(filename, func(index int, line string) error {
		var row []status
		for _, c := range line {
			s := sEmpty
			if c == '.' {
				s = sFloor
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
