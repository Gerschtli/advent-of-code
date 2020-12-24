package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestOrRuleImplementsRule(t *testing.T) {
	assert.Implements(t, (*Rule)(nil), OrRule{})
}

func TestOrRuleGetMatchesReturnsLengthForMatch(t *testing.T) {
	rules := map[int]Rule{
		0: OrRule{[][]int{{4, 1, 5}}},
		1: OrRule{[][]int{{2, 3}, {3, 2}}},
		2: OrRule{[][]int{{4, 4}, {5, 5}}},
		3: OrRule{[][]int{{4, 5}, {5, 4}}},
		4: ValueRule{'a'},
		5: ValueRule{'b'},
	}

	assert.Equal(t, []int{6}, rules[0].GetMatches(rules, "aaaabb", 0))
	assert.Equal(t, []int{4}, rules[1].GetMatches(rules, "aaaabb", 1))
}

func TestOrRuleGetMatchesReturnsNilWhenNoMatch(t *testing.T) {
	rules := map[int]Rule{
		0: OrRule{[][]int{{4, 1, 5}}},
		1: OrRule{[][]int{{2, 3}, {3, 2}}},
		2: OrRule{[][]int{{4, 4}, {5, 5}}},
		3: OrRule{[][]int{{4, 5}, {5, 4}}},
		4: ValueRule{'a'},
		5: ValueRule{'b'},
	}

	assert.Nil(t, rules[0].GetMatches(rules, "aaaabb", 1))
	assert.Nil(t, rules[1].GetMatches(rules, "aaaabb", 2))
}

func TestValueRuleImplementsRule(t *testing.T) {
	assert.Implements(t, (*Rule)(nil), ValueRule{})
}

func TestValueRuleGetMatchesReturnsLengthForMatch(t *testing.T) {
	rule := ValueRule{value: 'a'}
	rules := make(map[int]Rule)

	matches := rule.GetMatches(rules, "ab", 0)

	assert.Equal(t, []int{1}, matches)
}

func TestValueRuleGetMatchesReturnsNilWhenNoMatch(t *testing.T) {
	rule := ValueRule{value: 'a'}
	rules := make(map[int]Rule)

	assert.Nil(t, rule.GetMatches(rules, "ab", 1))
}

func TestValueRuleGetMatchesReturnsNilWhenIndexTooHigh(t *testing.T) {
	rule := ValueRule{value: 'a'}
	rules := make(map[int]Rule)

	assert.Nil(t, rule.GetMatches(rules, "ab", 2))
}
