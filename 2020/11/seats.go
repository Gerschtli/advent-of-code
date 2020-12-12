package main

type status int

const (
	statusFloor status = iota
	statusFree
	statusOccupied
)

type seats [][]status

func (s *seats) runRound(countOccupied func(y int, x int) int, triggerOccupied int) seats {
	var newSeats seats

	for i := range *s {
		var row []status
		for j, value := range (*s)[i] {
			newValue := value
			count := countOccupied(i, j)

			if value == statusFree && count == 0 {
				newValue = statusOccupied
			}
			if value == statusOccupied && count >= triggerOccupied {
				newValue = statusFree
			}

			row = append(row, newValue)
		}

		newSeats = append(newSeats, row)
	}

	return newSeats
}

func (s *seats) countOccupied(y int, x int) int {
	var count int

	for i := max(y-1, 0); i <= min(y+1, len(*s)-1); i++ {
		for j := max(x-1, 0); j <= min(x+1, len((*s)[i])-1); j++ {
			if i == y && j == x {
				continue
			}

			value := (*s)[i][j]

			if value == statusOccupied {
				count += 1
			}
		}
	}

	return count
}

func (s *seats) countAllOccupied() int {
	var count int
	for i := range *s {
		for _, value := range (*s)[i] {
			if value == statusOccupied {
				count += 1
			}
		}
	}

	return count
}

func min(i, i2 int) int {
	if i <= i2 {
		return i
	}

	return i2
}

func max(i, i2 int) int {
	if i >= i2 {
		return i
	}

	return i2
}
