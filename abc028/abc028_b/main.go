package main

import (
	"fmt"
)

func main() {
	var s string
	fmt.Scan(&s)

	charCounts := make([]int, 6)
	charIndexMap := map[string]int{
		"A": 0,
		"B": 1,
		"C": 2,
		"D": 3,
		"E": 4,
		"F": 5,
	}
	for _, char := range s {
		charCounts[charIndexMap[string(char)]]++
	}
	for i, count := range charCounts  {
		if i == len(charCounts)-1 {
			fmt.Printf("%d\n", count)
		} else {
			fmt.Printf("%d ", count)
		}
	}
}
