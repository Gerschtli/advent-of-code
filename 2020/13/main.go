package main

import (
	"errors"
	"math"
	"strconv"
	"strings"

	"github.com/Gerschtli/advent-of-code/lib/go/file"
)

func main() {
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
		factor := timestamp / b
		time := b * (factor + 1)
		if time < departureTime {
			bus, departureTime = b, time
		}
	}

	return bus, departureTime - timestamp
}
