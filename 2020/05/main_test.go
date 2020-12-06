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

func TestBuildSeatByCodeReturnsValidSeats(t *testing.T) {
	s, err := buildSeatByCode("BFFFBBFRRR")
	assert.Nil(t, err)
	assert.Equal(t, seat{70, 7}, s)

	s, err = buildSeatByCode("FFFBBBFRRR")
	assert.Nil(t, err)
	assert.Equal(t, seat{14, 7}, s)

	s, err = buildSeatByCode("BBFFBBFRLL")
	assert.Nil(t, err)
	assert.Equal(t, seat{102, 4}, s)
}

func TestBuildSeatByCodeReturnsErrorsForInvalidCodes(t *testing.T) {
	_, err := buildSeatByCode("BFFFBBFRR")
	if assert.NotNil(t, err) {
		assert.Equal(t, "code invalid: need 10 chars [BFFFBBFRR]", err.Error())
	}

	_, err = buildSeatByCode("FFFBBBRRRR")
	if assert.NotNil(t, err) {
		assert.Equal(t, "code invalid: invalid char, need F or B [FFFBBBRRRR]", err.Error())
	}

	_, err = buildSeatByCode("BFFFBBFLAR")
	if assert.NotNil(t, err) {
		assert.Equal(t, "code invalid: invalid char, need L or R [BFFFBBFLAR]", err.Error())
	}
}
