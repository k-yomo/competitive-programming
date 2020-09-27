package main

import (
	"fmt"
	"math"
	"strconv"
)

func main() {
	var n int
	fmt.Scan(&n)
	digits := len(strconv.Itoa(n))
	sum := 0
	for i := 1; i < digits; i += 2 {
		num := math.Pow(10, float64(i))
		sum += int(num) - (int(num) / 10)
	}
	if digits % 2 != 0 {
		sum += n - int(math.Pow(10, float64(digits - 1))) + 1
	}
	fmt.Println(sum)
}
