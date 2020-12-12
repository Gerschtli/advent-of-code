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

	assert.Len(t, lines, 1)
	assert.Empty(t, lines[0])
}

func TestParseSeatsReturnsSeats(t *testing.T) {
	seatsMap, err := parseSeats("./files/example.txt")
	expected := seats([][]status{
		{statusFree, statusFloor, statusFree, statusFree, statusFloor, statusFree, statusFree, statusFloor, statusFree, statusFree},
		{statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFloor, statusFree, statusFree},
		{statusFree, statusFloor, statusFree, statusFloor, statusFree, statusFloor, statusFloor, statusFree, statusFloor, statusFloor},
		{statusFree, statusFree, statusFree, statusFree, statusFloor, statusFree, statusFree, statusFloor, statusFree, statusFree},
		{statusFree, statusFloor, statusFree, statusFree, statusFloor, statusFree, statusFree, statusFloor, statusFree, statusFree},
		{statusFree, statusFloor, statusFree, statusFree, statusFree, statusFree, statusFree, statusFloor, statusFree, statusFree},
		{statusFloor, statusFloor, statusFree, statusFloor, statusFree, statusFloor, statusFloor, statusFloor, statusFloor, statusFloor},
		{statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree},
		{statusFree, statusFloor, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFloor, statusFree},
		{statusFree, statusFloor, statusFree, statusFree, statusFree, statusFree, statusFree, statusFloor, statusFree, statusFree},
	})

	assert.Nil(t, err)
	assert.Equal(t, expected, seatsMap)
}
