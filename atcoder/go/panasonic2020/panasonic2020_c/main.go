package main

import (
	"fmt"
)

func main() {
	var a, b, c float64
	fmt.Scan(&a, &b, &c)
	if 4*a*b < (c-a-b)*(c-a-b) && c-a-b > 0 {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
