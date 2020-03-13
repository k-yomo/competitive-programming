package main

import (
	"fmt"
)

func main() {
	var a, b int
	fmt.Scan(&a, &b)
	sum := a + b
	diff := a - b
	product := a * b
	if sum > diff && sum > product {
		fmt.Println(sum)
	} else if diff > sum && diff > product {
		fmt.Println(diff)
	} else {
		fmt.Println(product)
	}
}
