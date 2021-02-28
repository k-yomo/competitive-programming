package main

import (
	"fmt"
	"math"
	"strconv"
)

func main() {
	var a, b string
	fmt.Scan(&a, &b)
	n, _ := strconv.ParseFloat(a+b, 64)
	sqrt := math.Sqrt(n)
	if math.Floor(sqrt) == sqrt {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}