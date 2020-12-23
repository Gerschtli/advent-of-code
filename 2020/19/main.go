package main

import (
	"strconv"
	"strings"

	"github.com/Gerschtli/advent-of-code/lib/go/file"
)

func main() {
}

func parseNotes(filename string) (map[int]Rule, []string, error) {
	rules := make(map[int]Rule)
	var messages []string
	readMessages := false

	err := file.ReadFile(filename, func(index int, line string) error {
		if line == "" {
			readMessages = true
			return nil
		}

		if readMessages {
			messages = append(messages, line)
		} else {
			colonSplit := strings.Split(line, ":")
			id, err := strconv.Atoi(colonSplit[0])
			if err != nil {
				return err
			}

			var rule Rule
			if strings.ContainsRune(colonSplit[1], '"') {
				trim := strings.Trim(colonSplit[1], " \"")
				rule = ValueRule{trim[0]}
			} else {
				pipeSplit := strings.Split(colonSplit[1], "|")
				var rules [][]int
				for _, ps := range pipeSplit {
					spaceSplit := strings.Split(ps, " ")
					var ruleList []int
					for _, ss := range spaceSplit {
						if ss == "" {
							continue
						}
						r, err := strconv.Atoi(ss)
						if err != nil {
							return err
						}
						ruleList = append(ruleList, r)
					}
					rules = append(rules, ruleList)
				}

				rule = OrRule{rules}
			}

			rules[id] = rule
		}

		return nil
	})
	if err != nil {
		return nil, nil, err
	}

	return rules, messages, nil
}
