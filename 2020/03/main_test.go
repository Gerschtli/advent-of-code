package main

import (
	"bytes"
	"github.com/stretchr/testify/assert"
	"log"
	"os"
	"testing"
)

func TestMainPrintsTreeCount(t *testing.T) {
	var buf bytes.Buffer
	log.SetOutput(&buf)
	main()
	log.SetOutput(os.Stdout)

	assert.Contains(t, buf.String(), "189 trees found")
}

func TestLoadMapReturnsMap(t *testing.T) {
	m, err := loadMap("./files/map_example.txt")

	assert.Nil(t, err)
	assert.Equal(t, Map{
		{false, false, true, true, false, false},
		{true, false, false, false, true, false},
		{false, true, false, false, false, false},
		{false, false, true, false, true, false},
	}, m)
}

func TestLoadMapReturnsErrorWhenFileNotFound(t *testing.T) {
	_, err := loadMap("./files/map_not_found")

	assert.IsType(t, &os.PathError{}, err)
}

func TestLoadMapReturnsErrorWhenUnknownCharInMap(t *testing.T) {
	_, err := loadMap("./files/map_unknown_char.txt")

	if assert.NotNil(t, err) {
		assert.Equal(t, "unknown char found: '!'", err.Error())
	}
}

func TestLoadMapReturnsErrorWhenLinesNotEquallySized(t *testing.T) {
	_, err := loadMap("./files/map_line_length_mismatch.txt")

	if assert.NotNil(t, err) {
		assert.Equal(t, "provided map is not a rectangle", err.Error())
	}
}

func TestGetTreeCountReturnsCountForSpecificSlope(t *testing.T) {
	m := Map{
		{false, false, true},
		{true, false, false},
		{false, true, false},
		{false, false, true},
	}

	count := getTreeCount(&m, &Slope{2, 1})

	assert.Equal(t, 1, count)
}
