package main

type pocket map[int]map[int]map[int]bool

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
