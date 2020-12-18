package main

type plane struct {
	seats  [][]bool
	offset int
}

func buildPlane(maxRows int, maxColumns int) plane {
	seats := make([][]bool, maxRows)

	for r := range seats {
		seats[r] = make([]bool, maxColumns)
	}

	return plane{seats, 0}
}

func (p *plane) markSeatTaken(s *seat) {
	p.seats[s.row-p.offset][s.column] = true
}

func (p *plane) markFrontAndBackRowTaken() {
	backRow := len(p.seats) - 1
	columns := len(p.seats[0])
	for i := 0; i < columns; i++ {
		p.seats[0][i] = true
		p.seats[backRow][i] = true
	}
}

func (p *plane) shrinkPlaneSize() {
	length := len(p.seats)
	newFront, newBack := 0, length

	for _, row := range p.seats {
		if isRowFree(row) {
			newFront++
		} else {
			break
		}
	}

	for i := range p.seats {
		if isRowFree(p.seats[length-1-i]) {
			newBack--
		} else {
			break
		}
	}

	p.seats = p.seats[newFront:newBack]
	p.offset += newFront
}

func (p *plane) getFreeSeats() []seat {
	var seats []seat

	for r, row := range p.seats {
		for c, taken := range row {
			if !taken {
				seats = append(seats, seat{r + p.offset, c})
			}
		}
	}

	return seats
}

func isRowFree(row []bool) bool {
	for _, taken := range row {
		if taken {
			return false
		}
	}

	return true
}
