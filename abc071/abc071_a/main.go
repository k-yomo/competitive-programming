package main

import (
	"fmt"
	"math"
)

func AbsInt(n1 int, n2 int) int {
	return int(math.Abs(float64(n1 - n2)))
}

func main() {
	var x, a, b int
	fmt.Scan(&x, &a, &b)
	if AbsInt(x, a) < AbsInt(x, b) {
		fmt.Println("A")
	} else {
		fmt.Println("B")
	}
}
