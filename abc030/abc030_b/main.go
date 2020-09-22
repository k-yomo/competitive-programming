package main

import (
	"fmt"
	"math"
)

func main() {
	var n, m float64
	fmt.Scan(&n, &m)
	if n >= 12 {
		n -= 12
	}
	n *= 30 + m/60

	fmt.Println(AbsDiff(n, m))
}

func AbsDiff(n1 float64, n2 float64) int {
	return int(math.Abs(n1 - n2))
}

func Min(a, b float64) float64 {
	if a < b {
		return a
	}
	return b
}
