package main

import (
	"errors"
	"log"
	"math"
	"strconv"
	"strings"

	"github.com/Gerschtli/advent-of-code/lib/go/file"
)

func main() {
	timestamp, busses, err := parseNotes("./files/notes.txt")
	if err != nil {
		log.Fatal(err)
	}

	bus, waitingTime := findFirstBus(timestamp, busses)
	log.Printf("bus: %d, waiting time: %d, multiplied: %d\n", bus, waitingTime, bus*waitingTime)
}

func parseNotes(filename string) (int, []int, error) {
	var timestamp int
	var busses []int
	var counter int
	err := file.ReadFile(filename, func(index int, line string) error {
		if counter == 0 {
			timestampLocal, err := strconv.Atoi(line)
			if err != nil {
				return err
			}

			timestamp = timestampLocal
		} else if counter == 1 {
			for _, bus := range strings.Split(line, ",") {
				if bus == "x" {
					busses = append(busses, 0)
					continue
				}

				num, err := strconv.Atoi(bus)
				if err != nil {
					return err
				}

				busses = append(busses, num)
			}
		} else {
			return errors.New("too many lines in file")
		}

		counter++

		return nil
	})

	if err != nil {
		return 0, nil, err
	}

	return timestamp, busses, nil
}

func findFirstBus(timestamp int, busses []int) (int, int) {
	bus, departureTime := 0, math.MaxInt32

	for _, b := range busses {
		if b == 0 {
			continue
		}

		factor := timestamp / b
		time := b * (factor + 1)
		if time < departureTime {
			bus, departureTime = b, time
		}
	}

	return bus, departureTime - timestamp
}

func findEarliestTimestampWithMatchingOffsets(busses []int) int {
	for i := 0; ; i++ {
		timestamp := busses[0] * i
		if checkBusOffsets(busses, timestamp) {
			return timestamp
		}
	}
}

func checkBusOffsets(busses []int, timestamp int) bool {
	for j, bus := range busses {
		if bus == 0 {
			continue
		}
		if (timestamp+j)%bus != 0 {
			return false
		}
	}

	return true
}
