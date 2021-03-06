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
	assert.Contains(t, lines[0], "2020th number: 403")
	assert.Contains(t, lines[1], "30000000th number: 6823")
	assert.Empty(t, lines[2])
}

func TestGetNumber(t *testing.T) {
	assert.Equal(t, 0, getNumber([]int{0, 3, 6}, 10))
	assert.Equal(t, 1, getNumber([]int{1, 3, 2}, 2020))
	assert.Equal(t, 10, getNumber([]int{2, 1, 3}, 2020))
	assert.Equal(t, 27, getNumber([]int{1, 2, 3}, 2020))
	assert.Equal(t, 78, getNumber([]int{2, 3, 1}, 2020))
	assert.Equal(t, 438, getNumber([]int{3, 2, 1}, 2020))
	assert.Equal(t, 1836, getNumber([]int{3, 1, 2}, 2020))
}

func TestGetNumberWithBigNumbers(t *testing.T) {
	assert.Equal(t, 175594, getNumber([]int{0, 3, 6}, 30000000))
	assert.Equal(t, 2578, getNumber([]int{1, 3, 2}, 30000000))
	assert.Equal(t, 3544142, getNumber([]int{2, 1, 3}, 30000000))
	assert.Equal(t, 261214, getNumber([]int{1, 2, 3}, 30000000))
	assert.Equal(t, 6895259, getNumber([]int{2, 3, 1}, 30000000))
	assert.Equal(t, 18, getNumber([]int{3, 2, 1}, 30000000))
	assert.Equal(t, 362, getNumber([]int{3, 1, 2}, 30000000))
}
