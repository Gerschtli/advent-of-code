package main

import "math"

type pocketFixed map[int]pocket

func initPocketFixed(p pocket) pocketFixed {
	pf := make(pocketFixed)
	pf[0] = p
	return pf
}

func (pf *pocketFixed) countActive() int {
	count := 0

	for _, p := range *pf {
		count += p.countActive()
	}

	return count
}

func (pf *pocketFixed) countOfActiveNeighbors(w int) func(z, y, x int) int {
	return func(z, y, x int) int {
		count := 0
		for dw := w - 1; dw <= w+1; dw++ {
			p, ok := (*pf)[dw]
			if !ok {
				continue
			}
			count += p.countOfActiveNeighbors(dw == w)(z, y, x)
		}

		return count
	}
}

func (pf *pocketFixed) runCycle() pocketFixed {
	pfNew := make(pocketFixed)

	wLow, wHigh := math.MaxInt32, math.MinInt32
	pForRanges := pocket{}
	for i, p := range *pf {
		pForRanges = p
		if i < wLow {
			wLow = i
		}
		if i > wHigh {
			wHigh = i
		}
	}

	for w := wLow - 1; w <= wHigh+1; w++ {
		pNew := buildCycleResult(
			pForRanges.getRanges,
			func(z, y, x int) bool {
				value, ok := (*pf)[w][z][y][x]
				if !ok {
					value = false
				}
				return value
			},
			pf.countOfActiveNeighbors(w),
		)

		pfNew[w] = pNew
	}

	return pfNew
}
