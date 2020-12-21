package main

import (
	"log"

	"github.com/Gerschtli/advent-of-code/lib/go/file"
)

func main() {
	p, err := buildInitialPocket("./files/cubes.txt")
	if err != nil {
		log.Fatal(err)
	}

	pf := initPocketFixed(p)

	for i := 0; i < 6; i++ {
		p = p.runCycle(p.countOfActiveNeighbors(true))
		pf = pf.runCycle()
	}

	log.Printf("count after 6 cycles: %d\n", p.countActive())
	log.Printf("count fixed after 6 cycles: %d\n", pf.countActive())
}

func buildInitialPocket(filename string) (pocket, error) {
	p := make(pocket)
	p[0] = make(map[int]map[int]bool)
	err := file.ReadFile(filename, func(y int, line string) error {
		p[0][y] = make(map[int]bool)
		for x, c := range line {
			active := true
			if c == '.' {
				active = false
			}

			p[0][y][x] = active
		}
		return nil
	})
	if err != nil {
		return pocket{}, err
	}

	return p, err
}
