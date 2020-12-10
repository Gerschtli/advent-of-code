package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestOutputFindFirstInvalidReturnsInvalidNumber(t *testing.T) {
	o := output{
		[]int{
			35,
			20,
			15,
			25,
			47,
			40,
			62,
			55,
			65,
			95,
			102,
			117,
			150,
			182,
			127,
			219,
			299,
			277,
			309,
			576,
		},
		5,
	}

	firstInvalid, found := o.findFirstInvalid()

	assert.Equal(t, true, found)
	assert.Equal(t, 127, firstInvalid)
}

func TestOutputFindFirstInvalidReturnsFalseIfEveryNumberIsValid(t *testing.T) {
	o := output{
		[]int{
			1,
			2,
			3,
			4,
			6,
			10,
			13,
			10,
			19,
		},
		4,
	}

	_, found := o.findFirstInvalid()

	assert.Equal(t, false, found)
}

func TestOutputFindWeaknessReturnsWeaknessNumber(t *testing.T) {
	o := output{
		[]int{
			35,
			20,
			15,
			25,
			47,
			40,
			62,
			55,
			65,
			95,
			102,
			117,
			150,
			182,
			127,
			219,
			299,
			277,
			309,
			576,
		},
		5,
	}

	weakness, found := o.findWeakness(127)

	assert.Equal(t, true, found)
	assert.Equal(t, 62, weakness)
}

func TestOutputFindWeaknessReturnsFalseWhenNoWeaknessFound(t *testing.T) {
	o := output{
		[]int{
			1,
			2,
			3,
			4,
			6,
			11,
			9,
			10,
			19,
		},
		4,
	}

	_, found := o.findWeakness(11)

	assert.Equal(t, false, found)
}
