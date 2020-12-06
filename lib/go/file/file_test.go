package file

import (
	"errors"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestReadFileCallsHandlerForEachLine(t *testing.T) {
	var indexes []int
	var lines []string
	err := ReadFile("./files/example.txt", func(index int, line string) error {
		indexes = append(indexes, index)
		lines = append(lines, line)
		return nil
	})

	assert.Nil(t, err)
	assert.Equal(t, []int{0, 1, 2, 3}, indexes)
	assert.Equal(t, []string{"abc", "bcd", "", "cde"}, lines)
}

func TestReadFileReturnsErrorFromHandler(t *testing.T) {
	err := ReadFile("./files/example.txt", func(index int, line string) error {
		return errors.New("error in handler")
	})

	if assert.NotNil(t, err) {
		assert.Equal(t, "error in handler", err.Error())
	}
}

func TestReadFileReturnsErrorWhenFileNotFound(t *testing.T) {
	err := ReadFile("./files/example_not_found.txt", func(index int, line string) error {
		return nil
	})

	assert.IsType(t, &os.PathError{}, err)
}
