package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBuildPlaneInitialisesWithFalse(t *testing.T) {
	p := buildPlane(2, 2)

	assert.Equal(t, plane{seats: [][]bool{{false, false}, {false, false}}}, p)
}

func TestPlaneMarkSeatTaken(t *testing.T) {
	s := seat{1, 0}
	p := plane{seats: [][]bool{{false, false}, {false, false}}}

	p.markSeatTaken(&s)

	assert.Equal(t, true, p.seats[1][0])
}

func TestPlaneMarkSeatTakenWithOffset(t *testing.T) {
	s := seat{1, 0}
	p := plane{seats: [][]bool{{false, false}, {false, false}}, offset: 1}

	p.markSeatTaken(&s)

	assert.Equal(t, true, p.seats[0][0])
}

func TestPlaneGetFreeSeatsReturnsSliceOfSeats(t *testing.T) {
	p := plane{seats: [][]bool{{false, true}, {true, false}}, offset: 1}

	seats := p.getFreeSeats()

	assert.Equal(t, []seat{{1, 0}, {2, 1}}, seats)
}
