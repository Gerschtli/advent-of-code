package main

import (
	"os"
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLoadNumbersParsesFileContent(t *testing.T) {
	numbers, err := loadNumbers("./files/input_test")

	assert.Nil(t, err)
	assert.Equal(t, []int{1, 2, 3}, numbers)
}

func TestLoadNumbersReturnsErrorWhenFileContentIsNoInteger(t *testing.T) {
	_, err := loadNumbers("./files/input_invalid_test")

	assert.IsType(t, &strconv.NumError{}, err)
}

func TestLoadNumbersReturnsErrorWhenFileNotFound(t *testing.T) {
	_, err := loadNumbers("./files/not_found_test")

	assert.IsType(t, &os.PathError{}, err)
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
