package main

import (
	"fmt"
	"sort"
)

func main() {
	var s string
	var k int
	fmt.Scan(&s, &k)
	substrMap := map[string]bool{}
	substrMap[s] = true
	for i := 0; i < len(s)-1; i++ {
		substrMap[string(s[i])] = true
		for j := i+1; j < len(s); j++ {
			substr := s[i:j]
			if j == len(s)-1 {
				substrMap[string(s[j])] = true
			}
			substrMap[substr] = true
		}
	}
	var substrs []string
	for substr, _ := range substrMap {
		substrs = append(substrs, substr)
	}

	sort.Strings(substrs)
	fmt.Println(substrs[k-1])
}
