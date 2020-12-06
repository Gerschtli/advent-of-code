package main

import (
	"bytes"
	"log"
	"os"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMainLogsResults(t *testing.T) {
	var buf bytes.Buffer
	log.SetOutput(&buf)
	main()
	log.SetOutput(os.Stdout)

	lines := strings.Split(buf.String(), "\n")

	assert.Len(t, lines, 3)
	assert.Contains(t, lines[0], "highest seat id: 944")
	assert.Contains(t, lines[1], "free seat id: 554")
	assert.Empty(t, lines[2])
}

func TestReadSeatsReturnsSliceOfSeats(t *testing.T) {
	seats, err := readSeats("./files/example.txt")

	assert.Nil(t, err)
	assert.Equal(t, []seat{
		{70, 7},
		{14, 7},
		{102, 4},
	}, seats)
}

func TestReadSeatsReturnsErrorForInvalidInput(t *testing.T) {
	_, err := readSeats("./files/example_invalid.txt")

	if assert.NotNil(t, err) {
		assert.Equal(t, "code invalid: need 10 chars [FFFBBBFRR]", err.Error())
	}
}

func TestGetHighestSeatIdReturnsId(t *testing.T) {
	seats := []seat{
		{0, 0},
		{1, 0},
		{1, 2},
		{0, 1},
	}

	assert.Equal(t, 10, getHighestId(seats))
}
