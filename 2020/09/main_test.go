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
	assert.Contains(t, lines[0], "first invalid number: 248131121")
	assert.Contains(t, lines[1], "weakness: 31580383")
	assert.Empty(t, lines[2])
}

func TestParseFileReturnsSliceOfInts(t *testing.T) {
	numbers, err := parseFile("./files/example.txt")

	assert.Nil(t, err)
	assert.Equal(t, []int{35, 20, 15, 25, 47}, numbers)
}

func TestParseFileReturnsErrorForInvalidLink(t *testing.T) {
	_, err := parseFile("./files/example_invalid.txt")

	if assert.NotNil(t, err) {
		assert.Equal(t, "strconv.Atoi: parsing \"abc\": invalid syntax", err.Error())
	}
}
