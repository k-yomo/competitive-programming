package main

import "fmt"

func main() {
	var n, s, t, w int
	fmt.Scan(&n, &s, &t, &w)

	curWeight := w
	var bestBodyCount int
	if s <= curWeight && curWeight <= t {
		bestBodyCount++
	}

	for i := 0; i < n-1; i++ {
		var loseWeight int
		fmt.Scan(&loseWeight)
		curWeight += loseWeight
		if s <= curWeight && curWeight <= t {
			bestBodyCount++
		}
	}

	fmt.Println(bestBodyCount)
}
