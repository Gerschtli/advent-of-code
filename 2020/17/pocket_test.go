package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCountOfActiveNeighbors(t *testing.T) {
	p := pocket(map[int]map[int]map[int]bool{
		-1: {
			0: {0: true, 1: false, 2: false},
			1: {0: false, 1: false, 2: true},
			2: {0: false, 1: true, 2: false},
		},
		0: {
			0: {0: true, 1: false, 2: true},
			1: {0: false, 1: true, 2: true},
			2: {0: false, 1: true, 2: false},
		},
		1: {
			0: {0: true, 1: false, 2: false},
			1: {0: false, 1: false, 2: true},
			2: {0: false, 1: true, 2: false},
		},
	})

	assert.Equal(t, 10, p.countOfActiveNeighbors(0, 1, 1))
	assert.Equal(t, 2, p.countOfActiveNeighbors(-1, 0, 0))
}
