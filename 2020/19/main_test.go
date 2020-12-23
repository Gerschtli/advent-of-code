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

	assert.Len(t, lines, 2)
	assert.Contains(t, lines[0], "count of valid messages: 165")
	assert.Empty(t, lines[1])
}

func TestParseLines(t *testing.T) {
	rules, messages, err := parseNotes("./files/example.txt")

	assert.Nil(t, err)
	assert.Equal(t, map[int]Rule{
		0: OrRule{[][]int{{4, 1, 5}}},
		1: OrRule{[][]int{{2, 3}, {3, 2}}},
		2: OrRule{[][]int{{4, 4}, {5, 5}}},
		3: OrRule{[][]int{{4, 5}, {5, 4}}},
		4: ValueRule{'a'},
		5: ValueRule{'b'},
	}, rules)
	assert.Equal(t, []string{
		"ababbb",
		"bababa",
		"abbbab",
		"aaabbb",
		"aaaabbb",
	}, messages)
}
