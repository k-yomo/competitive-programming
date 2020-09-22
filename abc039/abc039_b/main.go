package main

import (
	"fmt"
	"math"
)

func main() {
	var x int
	fmt.Scan(&x)

	fmt.Println(int(math.Sqrt(math.Sqrt(float64(x)))))
}
