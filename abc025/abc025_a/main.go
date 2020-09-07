package main

import (
	"fmt"
	"sort"
)

func main() {
	var s string
	var n int
	fmt.Scan(&s, &n)
	var nicknameCandidates []string
	for _, a1 := range s {
		for _, b1 := range s {
			nicknameCandidates = append(nicknameCandidates, string(a1)+string(b1))
		}
	}
	sort.Strings(nicknameCandidates)
	fmt.Println(nicknameCandidates[n-1])
}
