package main

import (
	"fmt"
	"math"
	"sort"
)

func main() {
	var n int
	fmt.Scan(&n)
	circleRadii := make([]int, n)
	for i := range circleRadii {
		var radius int
		fmt.Scan(&radius)
		circleRadii[i] = radius
	}
	sort.Sort(sort.Reverse(sort.IntSlice(circleRadii)))
	ans := circleRadii[0] * circleRadii[0]
	for i, radius := range circleRadii[1:] {
		if i %2 == 0 {
			ans -= radius * radius
		} else {
			ans += radius * radius
		}
	}
	fmt.Println(float64(ans) * math.Pi)
}
