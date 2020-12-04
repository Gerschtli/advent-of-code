package main

// Map contains true for fields with trees and false for open squares. Further this 2-dimensional slice
// is a rectangle
type Map [][]bool

func (m *Map) get(p *Position) (isBlocked bool, bottomReached bool) {
	slice := *m
	if p.y >= len(slice) {
		return false, true
	}

	line := slice[p.y]
	return line[p.x%len(line)], false
}

type Slope struct {
	x int
	y int
}

type Position struct {
	x int
	y int
}

func (p *Position) move(s *Slope) Position {
	return Position{
		p.x + s.x,
		p.y + s.y,
	}
}
