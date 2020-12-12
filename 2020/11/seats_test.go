package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSeatsRunRoundReplacesFreeWithOccupiedWithFirstInitialData(t *testing.T) {
	seatsInitial := seats([][]status{
		{sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sEmpty, sFloor, sEmpty, sFloor, sEmpty, sFloor, sFloor, sEmpty, sFloor, sFloor},
		{sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sEmpty, sFloor, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sFloor, sFloor, sEmpty, sFloor, sEmpty, sFloor, sFloor, sFloor, sFloor, sFloor},
		{sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty},
		{sEmpty, sFloor, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty},
		{sEmpty, sFloor, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
	})

	seatsRound1 := seatsInitial.runRound(seatsInitial.countOccupied, 4)

	expected := seats([][]status{
		{sOccup, sFloor, sOccup, sOccup, sFloor, sOccup, sOccup, sFloor, sOccup, sOccup},
		{sOccup, sOccup, sOccup, sOccup, sOccup, sOccup, sOccup, sFloor, sOccup, sOccup},
		{sOccup, sFloor, sOccup, sFloor, sOccup, sFloor, sFloor, sOccup, sFloor, sFloor},
		{sOccup, sOccup, sOccup, sOccup, sFloor, sOccup, sOccup, sFloor, sOccup, sOccup},
		{sOccup, sFloor, sOccup, sOccup, sFloor, sOccup, sOccup, sFloor, sOccup, sOccup},
		{sOccup, sFloor, sOccup, sOccup, sOccup, sOccup, sOccup, sFloor, sOccup, sOccup},
		{sFloor, sFloor, sOccup, sFloor, sOccup, sFloor, sFloor, sFloor, sFloor, sFloor},
		{sOccup, sOccup, sOccup, sOccup, sOccup, sOccup, sOccup, sOccup, sOccup, sOccup},
		{sOccup, sFloor, sOccup, sOccup, sOccup, sOccup, sOccup, sOccup, sFloor, sOccup},
		{sOccup, sFloor, sOccup, sOccup, sOccup, sOccup, sOccup, sFloor, sOccup, sOccup},
	})

	assert.Equal(t, expected, seatsRound1)
}

func TestSeatsRunRoundFollowsRules(t *testing.T) {
	seatsRound1 := seats([][]status{
		{sOccup, sFloor, sOccup, sOccup, sFloor, sOccup, sOccup, sFloor, sOccup, sOccup},
		{sOccup, sOccup, sOccup, sOccup, sOccup, sOccup, sOccup, sFloor, sOccup, sOccup},
		{sOccup, sFloor, sOccup, sFloor, sOccup, sFloor, sFloor, sOccup, sFloor, sFloor},
		{sOccup, sOccup, sOccup, sOccup, sFloor, sOccup, sOccup, sFloor, sOccup, sOccup},
		{sOccup, sFloor, sOccup, sOccup, sFloor, sOccup, sOccup, sFloor, sOccup, sOccup},
		{sOccup, sFloor, sOccup, sOccup, sOccup, sOccup, sOccup, sFloor, sOccup, sOccup},
		{sFloor, sFloor, sOccup, sFloor, sOccup, sFloor, sFloor, sFloor, sFloor, sFloor},
		{sOccup, sOccup, sOccup, sOccup, sOccup, sOccup, sOccup, sOccup, sOccup, sOccup},
		{sOccup, sFloor, sOccup, sOccup, sOccup, sOccup, sOccup, sOccup, sFloor, sOccup},
		{sOccup, sFloor, sOccup, sOccup, sOccup, sOccup, sOccup, sFloor, sOccup, sOccup},
	})

	seatsRound2 := seatsRound1.runRound(seatsRound1.countOccupied, 4)

	expected := seats([][]status{
		{sOccup, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sOccup, sFloor, sOccup, sOccup},
		{sOccup, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty, sOccup},
		{sEmpty, sFloor, sEmpty, sFloor, sEmpty, sFloor, sFloor, sEmpty, sFloor, sFloor},
		{sOccup, sEmpty, sEmpty, sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sOccup},
		{sOccup, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sOccup, sFloor, sEmpty, sEmpty, sEmpty, sEmpty, sOccup, sFloor, sOccup, sOccup},
		{sFloor, sFloor, sEmpty, sFloor, sEmpty, sFloor, sFloor, sFloor, sFloor, sFloor},
		{sOccup, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sOccup},
		{sOccup, sFloor, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty},
		{sOccup, sFloor, sOccup, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sOccup, sOccup},
	})

	assert.Equal(t, expected, seatsRound2)
}

func TestSeatsCountAllOccupied(t *testing.T) {
	seatsRoundFinal := seats([][]status{
		{sOccup, sFloor, sOccup, sEmpty, sFloor, sEmpty, sOccup, sFloor, sOccup, sOccup},
		{sOccup, sEmpty, sEmpty, sEmpty, sOccup, sEmpty, sEmpty, sFloor, sEmpty, sOccup},
		{sEmpty, sFloor, sOccup, sFloor, sEmpty, sFloor, sFloor, sOccup, sFloor, sFloor},
		{sOccup, sEmpty, sOccup, sOccup, sFloor, sOccup, sOccup, sFloor, sEmpty, sOccup},
		{sOccup, sFloor, sOccup, sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sOccup, sFloor, sOccup, sEmpty, sOccup, sEmpty, sOccup, sFloor, sOccup, sOccup},
		{sFloor, sFloor, sEmpty, sFloor, sEmpty, sFloor, sFloor, sFloor, sFloor, sFloor},
		{sOccup, sEmpty, sOccup, sEmpty, sOccup, sOccup, sEmpty, sOccup, sEmpty, sOccup},
		{sOccup, sFloor, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty},
		{sOccup, sFloor, sOccup, sEmpty, sOccup, sEmpty, sOccup, sFloor, sOccup, sOccup},
	})

	count := seatsRoundFinal.countAllOccupied()

	assert.Equal(t, 37, count)
}
