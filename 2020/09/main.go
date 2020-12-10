package main

import (
	"log"
	"strconv"

	"github.com/Gerschtli/advent-of-code/lib/go/file"
)

func main() {
	numbers, err := parseFile("./files/output.txt")
	if err != nil {
		log.Fatal(err)
	}

	o := output{numbers, 25}
	firstInvalid, found := o.findFirstInvalid()
	if !found {
		log.Fatal("no invalid number found")
	}

	log.Printf("first invalid number: %d\n", firstInvalid)

	weakness, found := o.findWeakness(firstInvalid)
	if !found {
		log.Fatal("no weakness found")
	}

	log.Printf("weakness: %d\n", weakness)
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
