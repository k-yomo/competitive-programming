package main

import (
	"fmt"
)

func main() {
	var a, b, c, x, y int
	fmt.Scan(&a, &b, &c, &x, &y)
	var cost int
	if c*2 < a+b {
		n := Min(x, y)
		cost += c * 2 * n
		x -= n
		y -= n
	}
	if c*2 < a {
		cost += c * 2 * x
		x = 0
	}
	if c*2 < b {
		cost += c * 2 * y
		y = 0
	}
	cost += a * x
	cost += b * y

	fmt.Println(cost)
}

func Min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
