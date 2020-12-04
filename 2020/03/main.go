package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
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
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer func() {
		err := file.Close()
		if err != nil {
			log.Printf("error occured on file close: %v", err)
		}
	}()

	var m Map
	scanner := bufio.NewScanner(file)
	var length int
	for i := 0; scanner.Scan(); i++ {
		m = append(m, []bool{})

		for _, char := range scanner.Text() {
			var isBlocked bool

			switch char {
			case '.':
				isBlocked = false
			case '#':
				isBlocked = true
			default:
				return nil, errors.New(fmt.Sprintf("unknown char found: %q", char))
			}

			m[i] = append(m[i], isBlocked)
		}

		if i == 0 {
			length = len(m[i])
		} else if len(m[i]) != length {
			return nil, errors.New("provided map is not a rectangle")

		}
	}

	if err := scanner.Err(); err != nil {
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
