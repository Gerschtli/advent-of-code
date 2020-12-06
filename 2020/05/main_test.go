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
