package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPocketFixedCountActive(t *testing.T) {
	pf := pocketFixed(map[int]pocket{
		-1: {
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
		},
		0: {
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
		},
	})

	assert.Equal(t, 22, pf.countActive())
}

func TestPocketFixedCountOfActiveNeighbors(t *testing.T) {
	pf := pocketFixed(map[int]pocket{
		-1: {
			-1: {
				0: {0: true, 1: false, 2: false},
				1: {0: false, 1: false, 2: false},
				2: {0: false, 1: false, 2: false},
			},
			0: {
				0: {0: false, 1: false, 2: false},
				1: {0: false, 1: false, 2: false},
				2: {0: false, 1: false, 2: false},
			},
			1: {
				0: {0: false, 1: false, 2: false},
				1: {0: false, 1: false, 2: false},
				2: {0: false, 1: false, 2: true},
			},
		},
		0: {
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
		},
	})

	assert.Equal(t, 12, pf.countOfActiveNeighbors(0)(0, 1, 1))
	assert.Equal(t, 3, pf.countOfActiveNeighbors(0)(-1, 0, 0))
}

func TestPocketFixedRunCycle(t *testing.T) {
	pf := pocketFixed(map[int]pocket{
		0: {
			0: {
				0: {0: false, 1: true, 2: false},
				1: {0: false, 1: false, 2: true},
				2: {0: true, 1: true, 2: true},
			},
		},
	})

	pf2 := pf.runCycle()

	assert.Equal(t, pocketFixed(map[int]pocket{
		-1: {
			-1: {
				-1: {-1: false, 0: false, 1: false, 2: false, 3: false},
				0:  {-1: false, 0: false, 1: false, 2: false, 3: false},
				1:  {-1: false, 0: true, 1: false, 2: false, 3: false},
				2:  {-1: false, 0: false, 1: false, 2: true, 3: false},
				3:  {-1: false, 0: false, 1: true, 2: false, 3: false},
			},
			0: {
				-1: {-1: false, 0: false, 1: false, 2: false, 3: false},
				0:  {-1: false, 0: false, 1: false, 2: false, 3: false},
				1:  {-1: false, 0: true, 1: false, 2: false, 3: false},
				2:  {-1: false, 0: false, 1: false, 2: true, 3: false},
				3:  {-1: false, 0: false, 1: true, 2: false, 3: false},
			},
			1: {
				-1: {-1: false, 0: false, 1: false, 2: false, 3: false},
				0:  {-1: false, 0: false, 1: false, 2: false, 3: false},
				1:  {-1: false, 0: true, 1: false, 2: false, 3: false},
				2:  {-1: false, 0: false, 1: false, 2: true, 3: false},
				3:  {-1: false, 0: false, 1: true, 2: false, 3: false},
			},
		},
		0: {
			-1: {
				-1: {-1: false, 0: false, 1: false, 2: false, 3: false},
				0:  {-1: false, 0: false, 1: false, 2: false, 3: false},
				1:  {-1: false, 0: true, 1: false, 2: false, 3: false},
				2:  {-1: false, 0: false, 1: false, 2: true, 3: false},
				3:  {-1: false, 0: false, 1: true, 2: false, 3: false},
			},
			0: {
				-1: {-1: false, 0: false, 1: false, 2: false, 3: false},
				0:  {-1: false, 0: false, 1: false, 2: false, 3: false},
				1:  {-1: false, 0: true, 1: false, 2: true, 3: false},
				2:  {-1: false, 0: false, 1: true, 2: true, 3: false},
				3:  {-1: false, 0: false, 1: true, 2: false, 3: false},
			},
			1: {
				-1: {-1: false, 0: false, 1: false, 2: false, 3: false},
				0:  {-1: false, 0: false, 1: false, 2: false, 3: false},
				1:  {-1: false, 0: true, 1: false, 2: false, 3: false},
				2:  {-1: false, 0: false, 1: false, 2: true, 3: false},
				3:  {-1: false, 0: false, 1: true, 2: false, 3: false},
			},
		},
		1: {
			-1: {
				-1: {-1: false, 0: false, 1: false, 2: false, 3: false},
				0:  {-1: false, 0: false, 1: false, 2: false, 3: false},
				1:  {-1: false, 0: true, 1: false, 2: false, 3: false},
				2:  {-1: false, 0: false, 1: false, 2: true, 3: false},
				3:  {-1: false, 0: false, 1: true, 2: false, 3: false},
			},
			0: {
				-1: {-1: false, 0: false, 1: false, 2: false, 3: false},
				0:  {-1: false, 0: false, 1: false, 2: false, 3: false},
				1:  {-1: false, 0: true, 1: false, 2: false, 3: false},
				2:  {-1: false, 0: false, 1: false, 2: true, 3: false},
				3:  {-1: false, 0: false, 1: true, 2: false, 3: false},
			},
			1: {
				-1: {-1: false, 0: false, 1: false, 2: false, 3: false},
				0:  {-1: false, 0: false, 1: false, 2: false, 3: false},
				1:  {-1: false, 0: true, 1: false, 2: false, 3: false},
				2:  {-1: false, 0: false, 1: false, 2: true, 3: false},
				3:  {-1: false, 0: false, 1: true, 2: false, 3: false},
			},
		},
	}), pf2)
}
