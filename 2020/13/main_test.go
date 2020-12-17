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
	assert.Contains(t, lines[0], "bus: 23, waiting time: 5, multiplied: 115")
	assert.Contains(t, lines[1], "timestamp matching offsets: 756261495958122")
	assert.Empty(t, lines[2])
}

func TestParseNotesReturnsTimestampAndBusses(t *testing.T) {
	timestamp, busses, err := parseNotes("./files/example.txt")

	assert.Nil(t, err)
	assert.Equal(t, 939, timestamp)
	assert.Equal(t, []int{7, 13, 0, 0, 59, 0, 31, 19}, busses)
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
	bus, waitingTime := findFirstBus(939, []int{7, 13, 0, 0, 59, 0, 31, 19})

	assert.Equal(t, 59, bus)
	assert.Equal(t, 5, waitingTime)
}

func TestFindEarliestTimestampWithMatchingOffsetsWorksWithExamples(t *testing.T) {
	assert.Equal(t, 1068781, findEarliestTimestampWithMatchingOffsets([]int{7, 13, 0, 0, 59, 0, 31, 19}))
	assert.Equal(t, 3417, findEarliestTimestampWithMatchingOffsets([]int{17, 0, 13, 19}))
	assert.Equal(t, 754018, findEarliestTimestampWithMatchingOffsets([]int{67, 7, 59, 61}))
	assert.Equal(t, 779210, findEarliestTimestampWithMatchingOffsets([]int{67, 0, 7, 59, 61}))
	assert.Equal(t, 1261476, findEarliestTimestampWithMatchingOffsets([]int{67, 7, 0, 59, 61}))
	assert.Equal(t, 1202161486, findEarliestTimestampWithMatchingOffsets([]int{1789, 37, 47, 1889}))
}
