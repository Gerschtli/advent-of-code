package main

import (
	"errors"
	"fmt"
)

func main() {
}

type seat struct {
	row    int
	column int
}

func (s *seat) id() int {
	return s.row*8 + s.column
}

func buildSeatByCode(code string) (seat, error) {
	if len(code) != 10 {
		return seat{}, errors.New(fmt.Sprintf("code invalid: need 10 chars [%v]", code))
	}

	row, err := getCodeValue(code, code[:7], 127, 'F', 'B')
	if err != nil {
		return seat{}, err
	}

	column, err := getCodeValue(code, code[7:], 7, 'L', 'R')
	if err != nil {
		return seat{}, err
	}

	return seat{row, column}, nil
}

func getCodeValue(code string, codePart string, high int, charLow int32, charHigh int32) (int, error) {
	var low, step int
	for _, char := range codePart {
		step = (high - low + 1) / 2
		switch char {
		case charHigh:
			low += step
		case charLow:
			high -= step
		default:
			return 0, errors.New(fmt.Sprintf("code invalid: invalid char, need %c or %c [%v]", charLow, charHigh, code))
		}
	}

	return low, nil
}
