package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestMapGetReturnsValueForPosition(t *testing.T) {
	m := Map([][]bool{
		{true, false, false},
		{true, true, false},
		{false, false, true},
	})

	isBlocked, _ := m.get(&Position{0, 0})
	assert.Equal(t, true, isBlocked)
	isBlocked, _ = m.get(&Position{1, 0})
	assert.Equal(t, false, isBlocked)
	isBlocked, _ = m.get(&Position{1, 2})
	assert.Equal(t, false, isBlocked)
}

func TestMapGetReturnsValueForPositionWithToHighXValue(t *testing.T) {
	m := Map([][]bool{
		{true, false, false},
		{true, true, false},
		{false, false, true},
	})

	isBlocked, _ := m.get(&Position{3, 0})
	assert.Equal(t, true, isBlocked)
	isBlocked, _ = m.get(&Position{4, 0})
	assert.Equal(t, false, isBlocked)
	isBlocked, _ = m.get(&Position{7, 2})
	assert.Equal(t, false, isBlocked)
}

func TestMapGetReturnsBottomReachedAccordingToMapHeight(t *testing.T) {
	m := Map([][]bool{
		{true, false, false},
		{true, true, false},
		{false, false, true},
	})

	_, bottomReached := m.get(&Position{3, 2})
	assert.Equal(t, false, bottomReached)
	_, bottomReached = m.get(&Position{3, 3})
	assert.Equal(t, true, bottomReached)
}

func TestPositionMoveReturnsNewPosition(t *testing.T) {
	p := Position{1, 2}
	pNew := p.move()

	assert.Equal(t, 4, pNew.x)
	assert.Equal(t, 3, pNew.y)
}

func TestPositionMoveDoesNotChangeInitialPosition(t *testing.T) {
	p := Position{1, 2}
	p.move()

	assert.Equal(t, 1, p.x)
	assert.Equal(t, 2, p.y)
}
