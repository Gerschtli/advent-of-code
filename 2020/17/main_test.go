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
	assert.Contains(t, lines[0], "count after 6 cycles: 368")
	assert.Contains(t, lines[1], "count fixed after 6 cycles: 2696")
	assert.Empty(t, lines[2])
}

func TestBuildInitialPocket(t *testing.T) {
	p, err := buildInitialPocket("./files/example.txt")

	assert.Nil(t, err)
	assert.Equal(t, pocket(map[int]map[int]map[int]bool{
		0: {
			0: {
				0: false,
				1: true,
				2: false,
			},
			1: {
				0: false,
				1: false,
				2: true,
			},
			2: {
				0: true,
				1: true,
				2: true,
			},
		},
	}), p)
}
