package main

import "log"

func main() {
	starting := []int{16, 12, 1, 0, 15, 7, 11}
	number2020 := getNumber(starting, 2020)
	number30000000 := getNumber(starting, 30000000)

	log.Printf("2020th number: %d\n", number2020)
	log.Printf("30000000th number: %d\n", number30000000)
}

func getNumber(starting []int, limit int) int {
	numberIndexes := make(map[int]int)
	for i, v := range starting {
		if i == len(starting)-1 {
			break
		}
		numberIndexes[v] = i
	}

	lastNumber := starting[len(starting)-1]
	for i := len(starting); i < limit; i++ {
		lastIndex := i - 1
		lastOccurrence, found := numberIndexes[lastNumber]
		numberIndexes[lastNumber] = lastIndex

		if found {
			lastNumber = lastIndex - lastOccurrence
		} else {
			lastNumber = 0
		}
	}

	return lastNumber
}
