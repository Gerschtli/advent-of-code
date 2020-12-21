package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCountActive(t *testing.T) {
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

	assert.Equal(t, 11, p.countActive())
}

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

func TestRunCycle(t *testing.T) {
	p := pocket(map[int]map[int]map[int]bool{
		0: {
			0: {0: false, 1: true, 2: false},
			1: {0: false, 1: false, 2: true},
			2: {0: true, 1: true, 2: true},
		},
	})

	p2 := p.runCycle()

	assert.Equal(t, pocket(map[int]map[int]map[int]bool{
		-1: {
			1: {-1: false, 0: true, 1: false, 2: false, 3: false},
			2: {-1: false, 0: false, 1: false, 2: true, 3: false},
			3: {-1: false, 0: false, 1: true, 2: false, 3: false},
		},
		0: {
			1: {-1: false, 0: true, 1: false, 2: true, 3: false},
			2: {-1: false, 0: false, 1: true, 2: true, 3: false},
			3: {-1: false, 0: false, 1: true, 2: false, 3: false},
		},
		1: {
			1: {-1: false, 0: true, 1: false, 2: false, 3: false},
			2: {-1: false, 0: false, 1: false, 2: true, 3: false},
			3: {-1: false, 0: false, 1: true, 2: false, 3: false},
		},
	}), p2)
}
