package main

type plane [][]bool

func buildPlane(maxRows int, maxColumns int) plane {
	p := make([][]bool, maxRows, maxRows)

	for r := range p {
		p[r] = make([]bool, maxColumns, maxColumns)
	}

	return p
}

func (p *plane) markSeatTaken(s *seat) {
	(*p)[s.row][s.column] = true
}

func (p *plane) getFreeSeats() []seat {
	var seats []seat

	for r, row := range *p {
		for c, taken := range row {
			if !taken {
				seats = append(seats, seat{r, c})
			}
		}
	}

	return seats
}
