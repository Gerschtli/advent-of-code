package main

import (
	"log"
	"regexp"
	"strconv"

	"github.com/Gerschtli/advent-of-code/lib/go/file"
)

var rulesRegex = regexp.MustCompile(`(?:(\d+) )?(\w+ \w+) bags?`)

type bagRule struct {
	c     color
	count int
}
type rules map[color][]bagRule
type containable map[color][]color
type color string

func main() {
	ruleSet, err := readRules("./files/rules.txt")
	if err != nil {
		log.Fatal(err)
	}

	containableMap := transposeRules(&ruleSet)
	bagColorsForShinyGold := getTransitiveContainableForColor(&containableMap, "shiny gold")

	log.Printf("%d bag colors can contain shiny gold", len(bagColorsForShinyGold))

	bagCountShinyGold := getBagCount(&ruleSet, "shiny gold")

	log.Printf("%d bags are required to be in shiny gold", bagCountShinyGold)
}

func readRules(filename string) (rules, error) {
	ruleSet := rules(map[color][]bagRule{})

	err := file.ReadFile(filename, func(index int, line string) error {
		matches := rulesRegex.FindAllStringSubmatch(line, -1)
		key := color(matches[0][2])
		ruleSet[key] = []bagRule{}

		for _, match := range matches[1:] {
			if match[2] == "no other" {
				continue
			}
			count, _ := strconv.Atoi(match[1]) // does never fail because of regex
			ruleSet[key] = append(ruleSet[key], bagRule{color(match[2]), count})
		}

		return nil
	})

	if err != nil {
		return nil, err
	}

	return ruleSet, nil
}

func transposeRules(ruleSet *rules) containable {
	containableMap := containable(map[color][]color{})

	for key, values := range *ruleSet {
		for _, value := range values {
			containableMap[value.c] = append(containableMap[value.c], key)
		}
	}

	return containableMap
}

func getTransitiveContainableForColor(containableMap *containable, c color) map[color]bool {
	values, ok := (*containableMap)[c]
	if !ok {
		return nil
	}

	colors := map[color]bool{}
	for _, v := range values {
		colors[v] = true
		for key := range getTransitiveContainableForColor(containableMap, v) {
			colors[key] = true
		}
	}

	return colors
}

func getBagCount(ruleSet *rules, c color) int {
	bagRules, ok := (*ruleSet)[c]
	if !ok || len(bagRules) == 0 {
		return 0
	}

	var count int
	for _, r := range bagRules {
		count += r.count
		count += r.count * getBagCount(ruleSet, r.c)
	}

	return count
}
