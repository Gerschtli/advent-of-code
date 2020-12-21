package main

import "math"

type pocket map[int]map[int]map[int]bool

func (p *pocket) countActive() int {
	count := 0

	for _, plane := range *p {
		for _, row := range plane {
			for _, value := range row {
				if value {
					count++
				}
			}
		}
	}

	return count
}

func (p *pocket) countOfActiveNeighbors(z, y, x int) int {
	count := 0
	for dz := z - 1; dz <= z+1; dz++ {
		plane, ok := (*p)[dz]
		if !ok {
			continue
		}

		for dy := y - 1; dy <= y+1; dy++ {
			row, ok := plane[dy]
			if !ok {
				continue
			}

			for dx := x - 1; dx <= x+1; dx++ {
				value, ok := row[dx]
				if ok && value && !(dz == z && dy == y && dx == x) {
					count++
				}
			}
		}
	}

	return count
}

func (p *pocket) runCycle(countOfActiveNeighbors func(z, y, x int) int) pocket {
	pNew := make(pocket)

	// Yes, these next lovely lines look like this could be a bit refactored, but no!
	// Gotcha! This is Go: known for its simple syntax, expressiveness and hell of a boilerplate.. Here you go -.-
	zLow, zHigh := math.MaxInt32, math.MinInt32
	for i := range *p {
		if i < zLow {
			zLow = i
		}
		if i > zHigh {
			zHigh = i
		}
	}
	yLow, yHigh := math.MaxInt32, math.MinInt32
	for i := range (*p)[0] {
		if i < yLow {
			yLow = i
		}
		if i > yHigh {
			yHigh = i
		}
	}
	xLow, xHigh := math.MaxInt32, math.MinInt32
	for i := range (*p)[0][0] {
		if i < xLow {
			xLow = i
		}
		if i > xHigh {
			xHigh = i
		}
	}

	for z := zLow - 1; z <= zHigh+1; z++ {
		plane := make(map[int]map[int]bool)

		for y := yLow - 1; y <= yHigh+1; y++ {
			row := make(map[int]bool)

			for x := xLow - 1; x <= xHigh+1; x++ {
				value := getValue(z, y, x)

				countActiveNeighbors := countOfActiveNeighbors(z, y, x)

				newValue := value
				if value && countActiveNeighbors != 2 && countActiveNeighbors != 3 {
					newValue = false
				} else if !value && countActiveNeighbors == 3 {
					newValue = true
				}

				row[x] = newValue
			}

			plane[y] = row
		}

		pNew[z] = plane
	}

	return pNew
}
