package main

type Rule interface {
	GetMatches(rules map[int]Rule, input string, index int) []int
}

type OrRule struct {
	rules [][]int
}

func (o OrRule) GetMatches(rules map[int]Rule, input string, index int) []int {
	var matches []int
	for _, ruleList := range o.rules {
		lengthMatches := []int{0}

		for _, ruleId := range ruleList {
			var newLengths []int

			for _, length := range lengthMatches {
				for _, match := range rules[ruleId].GetMatches(rules, input, index+length) {
					newLengths = append(newLengths, length+match)
				}
			}

			lengthMatches = unique(newLengths)
		}

		matches = append(matches, lengthMatches...)
	}

	return unique(matches)
}

func unique(intSlice []int) []int {
	keys := make(map[int]bool)
	var list []int
	for _, entry := range intSlice {
		if _, value := keys[entry]; !value {
			keys[entry] = true
			list = append(list, entry)
		}
	}
	return list
}

type ValueRule struct {
	value uint8
}

func (v ValueRule) GetMatches(_ map[int]Rule, input string, index int) []int {
	if index < len(input) && input[index] == v.value {
		return []int{1}
	}
	return nil
}
