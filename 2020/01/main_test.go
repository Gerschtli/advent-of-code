package main

import (
	"bytes"
	"log"
	"os"
	"strconv"
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
	assert.Contains(t, lines[0], "found numbers (528, 1492), product is 787776")
	assert.Contains(t, lines[1], "found numbers (447, 611, 962), product is 262738554")
	assert.Empty(t, lines[2])
}

func TestLoadNumbersParsesFileContent(t *testing.T) {
	numbers, err := loadNumbers("./files/input_test.txt")

	assert.Nil(t, err)
	assert.Equal(t, []int{1, 2, 3}, numbers)
}

func TestLoadNumbersReturnsErrorWhenFileContentIsNoInteger(t *testing.T) {
	_, err := loadNumbers("./files/input_invalid_test.txt")

	assert.IsType(t, &strconv.NumError{}, err)
}

func TestFindMatchingPairReturnsPairWhenFound(t *testing.T) {
	numbers := []int{1721, 979, 366, 299, 675, 1456}
	number1, number2, err := findMatchingPair(numbers)

	assert.Nil(t, err)
	assert.Equal(t, 1721, number1)
	assert.Equal(t, 299, number2)
}

func TestFindMatchingPairReturnsErrorWhenNoPairFound(t *testing.T) {
	numbers := []int{1000, 1005, 1010, 1025}
	_, _, err := findMatchingPair(numbers)

	if assert.NotNil(t, err) {
		assert.Equal(t, "no matching numbers pair found", err.Error())
	}
}

func TestFindMatchingPairReturnsErrorWhenSliceIsEmpty(t *testing.T) {
	var numbers []int
	_, _, err := findMatchingPair(numbers)

	if assert.NotNil(t, err) {
		assert.Equal(t, "no matching numbers pair found", err.Error())
	}
}

func TestFindMatchingTripleReturnsTripleWhenFound(t *testing.T) {
	numbers := []int{1721, 979, 366, 299, 675, 1456}
	number1, number2, number3, err := findMatchingTriple(numbers)

	assert.Nil(t, err)
	assert.Equal(t, 979, number1)
	assert.Equal(t, 366, number2)
	assert.Equal(t, 675, number3)
}

func TestFindMatchingTripleReturnsErrorWhenNoTripleFound(t *testing.T) {
	numbers := []int{1000, 1005, 1010, 1025}
	_, _, _, err := findMatchingTriple(numbers)

	if assert.NotNil(t, err) {
		assert.Equal(t, "no matching numbers triple found", err.Error())
	}
}

func TestFindMatchingTripleReturnsErrorWhenSliceIsEmpty(t *testing.T) {
	var numbers []int
	_, _, _, err := findMatchingTriple(numbers)

	if assert.NotNil(t, err) {
		assert.Equal(t, "no matching numbers triple found", err.Error())
	}
}
