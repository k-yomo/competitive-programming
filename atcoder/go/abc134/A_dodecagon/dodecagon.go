package main

import (
	"fmt"
	"math"
)

func main() {
	var r float64
	fmt.Scan(&r)
	fmt.Println(3 * int(math.Pow(r, 2)))
}
