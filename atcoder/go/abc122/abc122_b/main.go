package main

import (
	"fmt"
	"sort"
	"strings"
)

func main() {
	var s string
	fmt.Scan(&s)
	chars := strings.Split(s, "")
	subStrLenList := make([]int, len(chars))
	curIndex := 0
	for _, char := range chars {
		if char == "A" || char == "C" || char == "G" || char == "T" {
			subStrLenList[curIndex] += 1
		} else {
			curIndex += 1
		}
	}
	sort.Ints(subStrLenList)
	fmt.Println(subStrLenList[len(subStrLenList)-1])
}
