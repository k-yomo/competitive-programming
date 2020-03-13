package main

import (
	"fmt"
	"math"
)

func main() {
	var a, b int64
	fmt.Scan(&a, &b)
	fmt.Println(math.Ceil(float64(a+b) / 2.0))
}
