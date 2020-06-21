package main

import (
	"fmt"
	"math"
)

func main() {
	var n int
	fmt.Scan(&n)

	h, w := 1, 1
	min := n
	for {
		if h*w <= n {
			sum := AbsDiff(h, w) + n - (h * w)
			if sum < min {
				min = sum
			}
			w++
		} else {
			h++
			w = 1
			if h*w > n {
				break
			}
		}
	}
	fmt.Println(min)
}

func AbsDiff(n1 int, n2 int) int {
	return int(math.Abs(float64(n1 - n2)))
}
