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

func TestParseNotesReturnsTimestampAndBusses(t *testing.T) {
	timestamp, busses, err := parseNotes("./files/example.txt")

	assert.Nil(t, err)
	assert.Equal(t, 939, timestamp)
	assert.Equal(t, []int{7, 13, 59, 31, 19}, busses)
}

func TestParseNotesReturnsErrorForInvalidBusValue(t *testing.T) {
	_, _, err := parseNotes("./files/example_invalid.txt")

	if assert.NotNil(t, err) {
		assert.Equal(t, "strconv.Atoi: parsing \"13a\": invalid syntax", err.Error())
	}
}

func TestParseNotesReturnsErrorForTooManyLines(t *testing.T) {
	_, _, err := parseNotes("./files/example_too_many_lines.txt")

	if assert.NotNil(t, err) {
		assert.Equal(t, "too many lines in file", err.Error())
	}
}

func TestFindFirstBusReturnsBusLineAndWaitingTime(t *testing.T) {
	bus, waitingTime := findFirstBus(939, []int{7, 13, 59, 31, 19})

	assert.Equal(t, 59, bus)
	assert.Equal(t, 5, waitingTime)
}
