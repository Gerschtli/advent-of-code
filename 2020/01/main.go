package main

import (
	"errors"
	"log"
	"strconv"

	"github.com/Gerschtli/advent-of-code/lib/go/file"
)

func main() {
	numbers, err := loadNumbers("./files/input")
	if err != nil {
		log.Fatal(err)
	}

	number1, number2, err := findMatchingPair(numbers)
	if err != nil {
		log.Fatal(err)
	}

	log.Printf("found numbers (%v, %v), product is %v\n", number1, number2, number1*number2)

	number1, number2, number3, err := findMatchingTriple(numbers)
	if err != nil {
		log.Fatal(err)
	}

	log.Printf("found numbers (%v, %v, %v), product is %v\n", number1, number2, number3, number1*number2*number3)
}

func loadNumbers(filename string) ([]int, error) {
	var numbers []int
	err := file.ReadFile(filename, func(index int, line string) error {
		value, err := strconv.Atoi(line)
		if err == nil {
			numbers = append(numbers, value)
		}
		return err
	})

	if err != nil {
		return nil, err
	}

	return numbers, nil
}

func findMatchingPair(numbers []int) (int, int, error) {
	numbersCount := len(numbers)

	for i := 0; i < numbersCount-1; i++ {
		for j := i + 1; j < numbersCount; j++ {
			if numbers[i]+numbers[j] == 2020 {
				return numbers[i], numbers[j], nil
			}
		}
	}

	return 0, 0, errors.New("no matching numbers pair found")
}

func findMatchingTriple(numbers []int) (int, int, int, error) {
	numbersCount := len(numbers)

	for i := 0; i < numbersCount-2; i++ {
		for j := i + 1; j < numbersCount-1; j++ {
			for k := j + 1; k < numbersCount; k++ {
				if numbers[i]+numbers[j]+numbers[k] == 2020 {
					return numbers[i], numbers[j], numbers[k], nil
				}
			}
		}
	}

	return 0, 0, 0, errors.New("no matching numbers triple found")
}
