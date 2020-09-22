package main

import (
	"fmt"
	"math"
)

func main() {
	var n, k float64
	fmt.Scan(&n, &k)
	fmt.Println(int(k * math.Pow(k-1, n-1)))
}
