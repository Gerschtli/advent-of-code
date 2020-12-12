package main

import (
	"reflect"
)

type status int

const (
	sFloor status = iota
	sEmpty
	sOccup
)

type seats [][]status

func (s *seats) runRound(countOccupied func(y int, x int) int, triggerOccupied int) seats {
	var newSeats seats

	for i := range *s {
		var row []status
		for j, value := range (*s)[i] {
			newValue := value
			count := countOccupied(i, j)

			if value == sEmpty && count == 0 {
				newValue = sOccup
			}
			if value == sOccup && count >= triggerOccupied {
				newValue = sEmpty
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

			if value == sOccup {
				count += 1
			}
		}
	}

	return count
}

func (s *seats) countOccupiedInSight(y int, x int) int {
	final := [3][3]bool{
		{true, true, true},
		{true, false, true},
		{true, true, true},
	}
	var end [3][3]bool
	var count int

	for i := 1; !reflect.DeepEqual(end, final); i++ {
		count += s.sub(&(end[0][0]), y-i, x-i)
		count += s.sub(&(end[0][1]), y-i, x)
		count += s.sub(&(end[0][2]), y-i, x+i)
		count += s.sub(&(end[1][0]), y, x-i)
		count += s.sub(&(end[1][2]), y, x+i)
		count += s.sub(&(end[2][0]), y+i, x-i)
		count += s.sub(&(end[2][1]), y+i, x)
		count += s.sub(&(end[2][2]), y+i, x+i)
	}

	return count
}

func (s *seats) sub(end *bool, y int, x int) int {
	if *end {
		return 0
	}

	if y < 0 || y >= len(*s) || x < 0 || x >= len((*s)[0]) {
		*end = true
		return 0
	}

	v := (*s)[y][x]
	if v != sFloor {
		*end = true
	}

	if v == sOccup {
		return 1
	}

	return 0
}

func (s *seats) countAllOccupied() int {
	var count int
	for i := range *s {
		for _, value := range (*s)[i] {
			if value == sOccup {
				count += 1
			}
		}
	}

	return count
}

func (s *seats) String() string {
	var str string

	for i := range *s {
		for _, value := range (*s)[i] {
			if value == sFloor {
				str += "."
			}
			if value == sEmpty {
				str += "L"
			}
			if value == sOccup {
				str += "#"
			}
		}
		str += "\n"
	}

	return str
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
