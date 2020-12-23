package main

type Rule interface {
	IsValid(rules map[int]Rule, input string, index int) (bool, int)
}

type OrRule struct {
	rules [][]int
}

func (o OrRule) IsValid(rules map[int]Rule, input string, index int) (bool, int) {
	for _, ruleList := range o.rules {
		lengthMatch := 0
		ruleListValid := true
		for _, ruleId := range ruleList {
			valid, length := rules[ruleId].IsValid(rules, input, index+lengthMatch)
			if !valid {
				ruleListValid = false
				break
			}
			lengthMatch += length
		}

		if ruleListValid {
			return true, lengthMatch
		}
	}

	return false, 0
}

type ValueRule struct {
	value uint8
}

func (v ValueRule) IsValid(_ map[int]Rule, input string, index int) (bool, int) {
	if index < len(input) && input[index] == v.value {
		return true, 1
	}
	return false, 0
}
