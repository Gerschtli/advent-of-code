package main

type plane struct {
	seats  [][]bool
	offset int
}

func buildPlane(maxRows int, maxColumns int) plane {
	seats := make([][]bool, maxRows, maxRows)

	for r := range seats {
		seats[r] = make([]bool, maxColumns, maxColumns)
	}

	return plane{seats, 0}
}

func (p *plane) markSeatTaken(s *seat) {
	p.seats[s.row-p.offset][s.column] = true
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
