package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	numbers, err := loadNumbers("2020/01/input")
	if err != nil {
		log.Fatal(err)
	}

	number1, number2, err := findMatchingNumbers(numbers)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("found numbers (%v, %v), product is %v\n", number1, number2, number1*number2)
}

func loadNumbers(filename string) ([]int, error) {
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

	var numbers []int
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		value, err := strconv.Atoi(scanner.Text())
		if err != nil {
			return nil, err
		}
		numbers = append(numbers, value)
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return numbers, nil
}

func findMatchingNumbers(numbers []int) (int, int, error) {
	numbersCount := len(numbers)

	for i := 0; i < numbersCount-1; i++ {
		for j := i + 1; j < numbersCount; j++ {
			if numbers[i]+numbers[j] == 2020 {
				return numbers[i], numbers[j], nil
			}
		}
	}

	return 0, 0, errors.New("no matching numbers found")
}
