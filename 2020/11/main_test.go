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
	assert.Contains(t, lines[0], "2448 occupied seats")
	assert.Contains(t, lines[1], "2234 occupied seats")
	assert.Empty(t, lines[2])
}

func TestParseSeatsReturnsSeats(t *testing.T) {
	seatsMap, err := parseSeats("./files/example.txt")
	expected := seats([][]status{
		{sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sEmpty, sFloor, sEmpty, sFloor, sEmpty, sFloor, sFloor, sEmpty, sFloor, sFloor},
		{sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sEmpty, sFloor, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sFloor, sFloor, sEmpty, sFloor, sEmpty, sFloor, sFloor, sFloor, sFloor, sFloor},
		{sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty},
		{sEmpty, sFloor, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty},
		{sEmpty, sFloor, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
	})

	assert.Nil(t, err)
	assert.Equal(t, expected, seatsMap)
}
