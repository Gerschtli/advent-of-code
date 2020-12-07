package main

import (
	"bytes"
	"log"
	"os"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMainLogsResults(t *testing.T) {
	var buf bytes.Buffer
	log.SetOutput(&buf)
	main()
	log.SetOutput(os.Stdout)

	lines := strings.Split(buf.String(), "\n")

	assert.Len(t, lines, 2)
	assert.Contains(t, lines[0], "131 bag colors can contain shiny gold")
	assert.Empty(t, lines[1])
}

func TestReadRulesReturnsParsedRuleSet(t *testing.T) {
	ruleSet, err := readRules("./files/example.txt")

	expected := rules(map[color][]bagRule{
		"light red":    {{"bright white", 0}, {"muted yellow", 0}},
		"dark orange":  {{"bright white", 0}, {"muted yellow", 0}},
		"bright white": {{"shiny gold", 0}},
		"muted yellow": {{"shiny gold", 0}, {"faded blue", 0}},
		"shiny gold":   {{"dark olive", 0}, {"vibrant plum", 0}},
		"dark olive":   {{"faded blue", 0}, {"dotted black", 0}},
		"vibrant plum": {{"faded blue", 0}, {"dotted black", 0}},
		"faded blue":   {},
		"dotted black": {},
	})

	assert.Nil(t, err)
	assert.Equal(t, expected, ruleSet)
}

func TestTransposeRulesReturnsContainable(t *testing.T) {
	rulesSet := rules(map[color][]bagRule{
		"light red":    {{"bright white", 0}, {"muted yellow", 0}},
		"dark orange":  {{"bright white", 0}, {"muted yellow", 0}},
		"bright white": {{"shiny gold", 0}},
		"muted yellow": {{"shiny gold", 0}, {"faded blue", 0}},
		"shiny gold":   {{"dark olive", 0}, {"vibrant plum", 0}},
		"dark olive":   {{"faded blue", 0}, {"dotted black", 0}},
		"vibrant plum": {{"faded blue", 0}, {"dotted black", 0}},
		"faded blue":   {},
		"dotted black": {},
	})

	containableMap := transposeRules(&rulesSet)

	assert.Len(t, containableMap, 7)
	assert.ElementsMatch(t, containableMap[color("bright white")], []color{"light red", "dark orange"})
	assert.ElementsMatch(t, containableMap[color("muted yellow")], []color{"light red", "dark orange"})
	assert.ElementsMatch(t, containableMap[color("shiny gold")], []color{"bright white", "muted yellow"})
	assert.ElementsMatch(t, containableMap[color("faded blue")], []color{"muted yellow", "dark olive", "vibrant plum"})
	assert.ElementsMatch(t, containableMap[color("dark olive")], []color{"shiny gold"})
	assert.ElementsMatch(t, containableMap[color("vibrant plum")], []color{"shiny gold"})
	assert.ElementsMatch(t, containableMap[color("dotted black")], []color{"dark olive", "vibrant plum"})
}

func TestGetTransitiveContainableForColorReturnsEmptySliceWhenNotFound(t *testing.T) {
	containableMap := containable(map[color][]color{})

	colors := getTransitiveContainableForColor(&containableMap, "green")

	assert.Empty(t, colors)
}

func TestGetTransitiveContainableForColorReturnsTransitiveColorList(t *testing.T) {
	containableMap := containable(map[color][]color{
		"bright white": {"light red", "dark orange"},
		"muted yellow": {"light red", "dark orange"},
		"shiny gold":   {"bright white", "muted yellow"},
		"faded blue":   {"muted yellow", "dark olive", "vibrant plum"},
		"dark olive":   {"shiny gold"},
		"vibrant plum": {"shiny gold"},
		"dotted black": {"dark olive", "vibrant plum"},
	})

	colors := getTransitiveContainableForColor(&containableMap, "shiny gold")

	assert.Equal(t, colors, map[color]bool{
		"bright white": true,
		"muted yellow": true,
		"light red":    true,
		"dark orange":  true,
	})
}
