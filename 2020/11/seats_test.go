package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSeatsRunRoundReplacesFreeWithOccupiedWithFirstInitialData(t *testing.T) {
	seatsInitial := seats([][]status{
		{statusFree, statusFloor, statusFree, statusFree, statusFloor, statusFree, statusFree, statusFloor, statusFree, statusFree},
		{statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFloor, statusFree, statusFree},
		{statusFree, statusFloor, statusFree, statusFloor, statusFree, statusFloor, statusFloor, statusFree, statusFloor, statusFloor},
		{statusFree, statusFree, statusFree, statusFree, statusFloor, statusFree, statusFree, statusFloor, statusFree, statusFree},
		{statusFree, statusFloor, statusFree, statusFree, statusFloor, statusFree, statusFree, statusFloor, statusFree, statusFree},
		{statusFree, statusFloor, statusFree, statusFree, statusFree, statusFree, statusFree, statusFloor, statusFree, statusFree},
		{statusFloor, statusFloor, statusFree, statusFloor, statusFree, statusFloor, statusFloor, statusFloor, statusFloor, statusFloor},
		{statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree},
		{statusFree, statusFloor, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFloor, statusFree},
		{statusFree, statusFloor, statusFree, statusFree, statusFree, statusFree, statusFree, statusFloor, statusFree, statusFree},
	})

	seatsRound1 := seatsInitial.runRound()

	expected := seats([][]status{
		{statusOccupied, statusFloor, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied},
		{statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied},
		{statusOccupied, statusFloor, statusOccupied, statusFloor, statusOccupied, statusFloor, statusFloor, statusOccupied, statusFloor, statusFloor},
		{statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied},
		{statusOccupied, statusFloor, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied},
		{statusOccupied, statusFloor, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied},
		{statusFloor, statusFloor, statusOccupied, statusFloor, statusOccupied, statusFloor, statusFloor, statusFloor, statusFloor, statusFloor},
		{statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied},
		{statusOccupied, statusFloor, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusFloor, statusOccupied},
		{statusOccupied, statusFloor, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied},
	})

	assert.Equal(t, expected, seatsRound1)
}

func TestSeatsRunRoundFollowsRules(t *testing.T) {
	seatsRound1 := seats([][]status{
		{statusOccupied, statusFloor, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied},
		{statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied},
		{statusOccupied, statusFloor, statusOccupied, statusFloor, statusOccupied, statusFloor, statusFloor, statusOccupied, statusFloor, statusFloor},
		{statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied},
		{statusOccupied, statusFloor, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied},
		{statusOccupied, statusFloor, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied},
		{statusFloor, statusFloor, statusOccupied, statusFloor, statusOccupied, statusFloor, statusFloor, statusFloor, statusFloor, statusFloor},
		{statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied},
		{statusOccupied, statusFloor, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusFloor, statusOccupied},
		{statusOccupied, statusFloor, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied},
	})

	seatsRound2 := seatsRound1.runRound()

	expected := seats([][]status{
		{statusOccupied, statusFloor, statusFree, statusFree, statusFloor, statusFree, statusOccupied, statusFloor, statusOccupied, statusOccupied},
		{statusOccupied, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFloor, statusFree, statusOccupied},
		{statusFree, statusFloor, statusFree, statusFloor, statusFree, statusFloor, statusFloor, statusFree, statusFloor, statusFloor},
		{statusOccupied, statusFree, statusFree, statusFree, statusFloor, statusFree, statusFree, statusFloor, statusFree, statusOccupied},
		{statusOccupied, statusFloor, statusFree, statusFree, statusFloor, statusFree, statusFree, statusFloor, statusFree, statusFree},
		{statusOccupied, statusFloor, statusFree, statusFree, statusFree, statusFree, statusOccupied, statusFloor, statusOccupied, statusOccupied},
		{statusFloor, statusFloor, statusFree, statusFloor, statusFree, statusFloor, statusFloor, statusFloor, statusFloor, statusFloor},
		{statusOccupied, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusOccupied},
		{statusOccupied, statusFloor, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFloor, statusFree},
		{statusOccupied, statusFloor, statusOccupied, statusFree, statusFree, statusFree, statusFree, statusFloor, statusOccupied, statusOccupied},
	})

	assert.Equal(t, expected, seatsRound2)
}

func TestSeatsCountAllOccupied(t *testing.T) {
	seatsRoundFinal := seats([][]status{
		{statusOccupied, statusFloor, statusOccupied, statusFree, statusFloor, statusFree, statusOccupied, statusFloor, statusOccupied, statusOccupied},
		{statusOccupied, statusFree, statusFree, statusFree, statusOccupied, statusFree, statusFree, statusFloor, statusFree, statusOccupied},
		{statusFree, statusFloor, statusOccupied, statusFloor, statusFree, statusFloor, statusFloor, statusOccupied, statusFloor, statusFloor},
		{statusOccupied, statusFree, statusOccupied, statusOccupied, statusFloor, statusOccupied, statusOccupied, statusFloor, statusFree, statusOccupied},
		{statusOccupied, statusFloor, statusOccupied, statusFree, statusFloor, statusFree, statusFree, statusFloor, statusFree, statusFree},
		{statusOccupied, statusFloor, statusOccupied, statusFree, statusOccupied, statusFree, statusOccupied, statusFloor, statusOccupied, statusOccupied},
		{statusFloor, statusFloor, statusFree, statusFloor, statusFree, statusFloor, statusFloor, statusFloor, statusFloor, statusFloor},
		{statusOccupied, statusFree, statusOccupied, statusFree, statusOccupied, statusOccupied, statusFree, statusOccupied, statusFree, statusOccupied},
		{statusOccupied, statusFloor, statusFree, statusFree, statusFree, statusFree, statusFree, statusFree, statusFloor, statusFree},
		{statusOccupied, statusFloor, statusOccupied, statusFree, statusOccupied, statusFree, statusOccupied, statusFloor, statusOccupied, statusOccupied},
	})

	count := seatsRoundFinal.countAllOccupied()

	assert.Equal(t, 37, count)
}
