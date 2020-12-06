package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBuildPlaneInitialisesWithFalse(t *testing.T) {
	p := buildPlane(2, 2)

	assert.Equal(t, plane([][]bool{{false, false}, {false, false}}), p)
}

func TestPlaneMarkSeatTaken(t *testing.T) {
	s := seat{1, 0}
	p := plane([][]bool{{false, false}, {false, false}})

	p.markSeatTaken(&s)

	assert.Equal(t, true, p[1][0])
}

func TestPlaneGetFreeSeatsReturnsSliceOfSeats(t *testing.T) {
	p := plane([][]bool{{false, true}, {true, false}})

	seats := p.getFreeSeats()

	assert.Equal(t, []seat{{0, 0}, {1, 1}}, seats)
}
