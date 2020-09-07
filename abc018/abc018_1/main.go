package main

import (
	"fmt"
	"sort"
)

func main() {
	var a, b, c int
	fmt.Scan(&a, &b, &c)

	scores := []int{a, b, c}
	sort.Sort(sort.Reverse(sort.IntSlice(scores)))
	scoreRankMap := map[int]int{}
	for i, score := range scores {
		scoreRankMap[score] = i + 1
	}

	fmt.Println(scoreRankMap[a])
	fmt.Println(scoreRankMap[b])
	fmt.Println(scoreRankMap[c])
}
