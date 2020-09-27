package main

import (
	"fmt"
	"math"
)

func main() {
	var x int
	fmt.Scan(&x)

	balance := 100
	year := 0
	for balance < x {
		balance = int(math.Floor(float64(balance) * 1.01))
		year++
	}
	fmt.Println(year)
}
