package main

import (
	"errors"
	"fmt"
	"log"

	"github.com/Gerschtli/advent-of-code/lib/go/file"
)

func main() {
	m, err := loadMap("./files/map.txt")
	if err != nil {
		log.Fatal(err)
	}

	log.Printf("%v trees found", getTreeCount(&m, &Slope{3, 1}))

	slopes := []Slope{
		{1, 1},
		{3, 1},
		{5, 1},
		{7, 1},
		{1, 2},
	}
	product := 1
	for _, slope := range slopes {
		product *= getTreeCount(&m, &slope)
	}

	log.Printf("product of all trees found: %v", product)
}

func loadMap(filename string) (Map, error) {
	var m Map
	var length int
	err := file.ReadFile(filename, func(index int, line string) error {
		m = append(m, []bool{})

		for _, char := range line {
			var isBlocked bool

			switch char {
			case '.':
				isBlocked = false
			case '#':
				isBlocked = true
			default:
				return errors.New(fmt.Sprintf("unknown char found: %q", char))
			}

			m[index] = append(m[index], isBlocked)
		}

		if index == 0 {
			length = len(m[index])
		} else if len(m[index]) != length {
			return errors.New("provided map is not a rectangle")
		}

		return nil
	})

	if err != nil {
		return nil, err
	}

	return m, nil
}

func getTreeCount(m *Map, slope *Slope) int {
	position := Position{}
	count := 0

	for {
		isBlocked, bottomReached := m.get(&position)
		if bottomReached {
			break
		}

		if isBlocked {
			count++
		}

		position = position.move(slope)
	}

	return count
}
