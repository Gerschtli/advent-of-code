package main

type output struct {
	numbers        []int
	preambleLength int
}

func (o *output) findFirstInvalid() (int, bool) {
	for i := o.preambleLength; i < len(o.numbers); i++ {
		if !o.isValid(i) {
			return o.numbers[i], true
		}
	}

	return 0, false
}

func (o *output) isValid(index int) bool {
	for i := index - o.preambleLength; i < index-1; i++ {
		for j := i + 1; j < index; j++ {
			if o.numbers[i]+o.numbers[j] == o.numbers[index] {
				return true
			}
		}
	}

	return false
}
