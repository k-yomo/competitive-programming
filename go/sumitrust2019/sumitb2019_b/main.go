package main

import (
	"fmt"
	"math"
)

func main() {
	var n float64
	fmt.Scan(&n)
	x := n / 1.08
	if math.Floor(math.Ceil(x)*1.08) == n {
		fmt.Println(math.Ceil(x))
	} else {
		fmt.Println(":(")
	}
}
