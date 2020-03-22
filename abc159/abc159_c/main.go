package main

import (
	"fmt"
	"math"
)

func main() {
	var l float64
	fmt.Scan(&l)
	fmt.Printf("%.12f\n", math.Pow(l/3, 3))
}
