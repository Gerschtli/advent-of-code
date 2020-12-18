package main

import "log"

func main() {
	number := getNumber([]int{16, 12, 1, 0, 15, 7, 11}, 2020)
	log.Printf("2020th number: %d\n", number)
}

func getNumber(starting []int, limit int) int {
	numbers := starting

	for i := len(starting) - 1; i < limit; i++ {
		lastNumber := numbers[i]
		lastOccurrence, found := getLastOccurrence(lastNumber, numbers[:i])
		if found {
			numbers = append(numbers, i-lastOccurrence)
		} else {
			numbers = append(numbers, 0)
		}
	}

	return numbers[limit-1]
}

func getLastOccurrence(needle int, haystack []int) (int, bool) {
	for i := len(haystack) - 1; i >= 0; i-- {
		if haystack[i] == needle {
			return i, true
		}
	}

	return 0, false
}
