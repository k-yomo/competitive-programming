package main

import (
	"fmt"
)

func main() {
	var n int
	fmt.Scan(&n)

	flowerCount := map[int]bool{}
	var pollinationCount int
	for i := 0; i < n; i++ {
		var flower int
		fmt.Scan(&flower)
		if flowerCount[flower] {
			pollinationCount++
		}
		flowerCount[flower] = true
	}

	fmt.Println(pollinationCount)
}

