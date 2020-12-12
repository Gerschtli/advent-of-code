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

func TestSeatsRunRoundFollowsRulesOfPart2(t *testing.T) {
	seatsRound2 := seats([][]status{
		{sOccup, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sOccup},
		{sOccup, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sEmpty, sFloor, sEmpty, sFloor, sEmpty, sFloor, sFloor, sEmpty, sFloor, sFloor},
		{sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sEmpty, sFloor, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty, sEmpty},
		{sFloor, sFloor, sEmpty, sFloor, sEmpty, sFloor, sFloor, sFloor, sFloor, sFloor},
		{sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sOccup},
		{sOccup, sFloor, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty},
		{sOccup, sFloor, sEmpty, sEmpty, sEmpty, sEmpty, sEmpty, sFloor, sEmpty, sOccup},
	})

	seatsRound3 := seatsRound2.runRound(seatsRound2.countOccupiedInSight, 5)

	expected := seats([][]status{
		{sOccup, sFloor, sEmpty, sOccup, sFloor, sOccup, sOccup, sFloor, sEmpty, sOccup},
		{sOccup, sEmpty, sOccup, sOccup, sOccup, sOccup, sOccup, sFloor, sEmpty, sEmpty},
		{sEmpty, sFloor, sOccup, sFloor, sOccup, sFloor, sFloor, sOccup, sFloor, sFloor},
		{sOccup, sOccup, sEmpty, sOccup, sFloor, sOccup, sOccup, sFloor, sOccup, sOccup},
		{sOccup, sFloor, sOccup, sOccup, sFloor, sOccup, sEmpty, sFloor, sOccup, sOccup},
		{sOccup, sFloor, sOccup, sOccup, sOccup, sOccup, sOccup, sFloor, sOccup, sEmpty},
		{sFloor, sFloor, sOccup, sFloor, sOccup, sFloor, sFloor, sFloor, sFloor, sFloor},
		{sEmpty, sEmpty, sEmpty, sOccup, sOccup, sOccup, sOccup, sEmpty, sEmpty, sOccup},
		{sOccup, sFloor, sEmpty, sOccup, sOccup, sOccup, sOccup, sOccup, sFloor, sEmpty},
		{sOccup, sFloor, sEmpty, sOccup, sOccup, sOccup, sOccup, sFloor, sEmpty, sOccup},
	})

	assert.Equal(t, expected, seatsRound3)
}

func TestCountOccupiedInSightExample1(t *testing.T) {
	seatsMap := seats([][]status{
		{sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sOccup, sFloor},
		{sFloor, sFloor, sFloor, sOccup, sFloor, sFloor, sFloor, sFloor, sFloor},
		{sFloor, sOccup, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor},
		{sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor},
		{sFloor, sFloor, sOccup, sEmpty, sFloor, sFloor, sFloor, sFloor, sOccup},
		{sFloor, sFloor, sFloor, sFloor, sOccup, sFloor, sFloor, sFloor, sFloor},
		{sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor},
		{sOccup, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor},
		{sFloor, sFloor, sFloor, sOccup, sFloor, sFloor, sFloor, sFloor, sFloor},
	})

	assert.Equal(t, 8, seatsMap.countOccupiedInSight(4, 3))
}

func TestCountOccupiedInSightExample2(t *testing.T) {
	seatsMap := seats([][]status{
		{sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor},
		{sFloor, sEmpty, sFloor, sEmpty, sFloor, sOccup, sFloor, sOccup, sFloor, sOccup, sFloor, sOccup, sFloor},
		{sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor, sFloor},
	})

	assert.Equal(t, 0, seatsMap.countOccupiedInSight(1, 1))
	assert.Equal(t, 1, seatsMap.countOccupiedInSight(1, 3))
}

func TestCountOccupiedInSightExample3(t *testing.T) {
	seatsMap := seats([][]status{
		{sFloor, sOccup, sOccup, sFloor, sOccup, sOccup, sFloor},
		{sOccup, sFloor, sOccup, sFloor, sOccup, sFloor, sOccup},
		{sOccup, sOccup, sFloor, sFloor, sFloor, sOccup, sOccup},
		{sFloor, sFloor, sFloor, sEmpty, sFloor, sFloor, sFloor},
		{sOccup, sOccup, sFloor, sFloor, sFloor, sOccup, sOccup},
		{sOccup, sFloor, sOccup, sFloor, sOccup, sFloor, sOccup},
		{sFloor, sOccup, sOccup, sFloor, sOccup, sOccup, sFloor},
	})

	assert.Equal(t, 0, seatsMap.countOccupiedInSight(3, 3))
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
