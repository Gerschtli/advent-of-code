package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestOrRuleImplementsRule(t *testing.T) {
	assert.Implements(t, (*Rule)(nil), OrRule{})
}

func TestOrRuleIsValidReturnsTrueForMatch(t *testing.T) {
	rules := map[int]Rule{
		0: OrRule{[][]int{{4, 1, 5}}},
		1: OrRule{[][]int{{2, 3}, {3, 2}}},
		2: OrRule{[][]int{{4, 4}, {5, 5}}},
		3: OrRule{[][]int{{4, 5}, {5, 4}}},
		4: ValueRule{'a'},
		5: ValueRule{'b'},
	}

	valid, length := rules[0].IsValid(rules, "aaaabb", 0)
	assert.Equal(t, true, valid)
	assert.Equal(t, 6, length)

	valid, length = rules[1].IsValid(rules, "aaaabb", 1)
	assert.Equal(t, true, valid)
	assert.Equal(t, 4, length)
}

func TestOrRuleIsValidReturnsFalseWhenNoMatch(t *testing.T) {
	rules := map[int]Rule{
		0: OrRule{[][]int{{4, 1, 5}}},
		1: OrRule{[][]int{{2, 3}, {3, 2}}},
		2: OrRule{[][]int{{4, 4}, {5, 5}}},
		3: OrRule{[][]int{{4, 5}, {5, 4}}},
		4: ValueRule{'a'},
		5: ValueRule{'b'},
	}

	valid, length := rules[0].IsValid(rules, "aaaabb", 1)
	assert.Equal(t, false, valid)
	assert.Equal(t, 0, length)

	valid, length = rules[1].IsValid(rules, "aaaabb", 2)
	assert.Equal(t, false, valid)
	assert.Equal(t, 0, length)
}

func TestValueRuleImplementsRule(t *testing.T) {
	assert.Implements(t, (*Rule)(nil), ValueRule{})
}

func TestValueRuleIsValidReturnsTrueForMatch(t *testing.T) {
	rule := ValueRule{value: 'a'}
	rules := make(map[int]Rule)

	valid, length := rule.IsValid(rules, "ab", 0)

	assert.Equal(t, true, valid)
	assert.Equal(t, 1, length)
}

func TestValueRuleIsValidReturnsFalseWhenNoMatch(t *testing.T) {
	rule := ValueRule{value: 'a'}
	rules := make(map[int]Rule)

	valid, length := rule.IsValid(rules, "ab", 1)

	assert.Equal(t, false, valid)
	assert.Equal(t, 0, length)
}
