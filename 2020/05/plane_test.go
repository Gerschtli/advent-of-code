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

func TestPlaneMarkFrontAndBackRowTaken(t *testing.T) {
	p := plane{seats: [][]bool{
		{true, false, false},
		{false, true, false},
		{false, false, true},
	}, offset: 1}
	expected := plane{seats: [][]bool{
		{true, true, true},
		{false, true, false},
		{true, true, true},
	}, offset: 1}

	p.markFrontAndBackRowTaken()

	assert.Equal(t, expected, p)
}

func TestPlaneShrinkPlaneSizeRemovesEmptyRowsAtFrontAndBack(t *testing.T) {
	p := plane{seats: [][]bool{
		{false, false, false},
		{true, false, false},
		{false, false, false},
		{false, true, false},
		{false, false, false},
	}, offset: 1}
	expected := plane{seats: [][]bool{
		{true, false, false},
		{false, false, false},
		{false, true, false},
	}, offset: 2}

	p.shrinkPlaneSize()

	assert.Equal(t, expected, p)
}

func TestPlaneGetFreeSeatsReturnsSliceOfSeats(t *testing.T) {
	p := plane{seats: [][]bool{{false, true}, {true, false}}, offset: 1}

	seats := p.getFreeSeats()

	assert.Equal(t, []seat{{1, 0}, {2, 1}}, seats)
}
