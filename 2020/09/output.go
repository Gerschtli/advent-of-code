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

func (o *output) findWeakness(invalidNumber int) (int, bool) {
	length := len(o.numbers)

	for i := 0; i < length-1; i++ {
		firstValue := o.numbers[i]
		smallest, largest, sum := firstValue, firstValue, firstValue

		for j := i + 1; j < length; j++ {
			lastValue := o.numbers[j]
			sum += lastValue

			if lastValue < smallest {
				smallest = lastValue
			}
			if lastValue > largest {
				largest = lastValue
			}

			if sum == invalidNumber {
				return smallest + largest, true
			}
			if sum > invalidNumber {
				break
			}
		}
	}

	return 0, false
}
