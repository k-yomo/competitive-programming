package main

import (
	"fmt"
	"math"
)

func main() {
	var a, b int
	fmt.Scan(&a, &b)

	fmt.Println(Min(AbsDiff(a, b), Min(AbsDiff(a+10, b), AbsDiff(a, b+10))))

}

func Min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func AbsDiff(n1 int, n2 int) int {
	return int(math.Abs(float64(n1 - n2)))
}
