package main

import (
	"fmt"
)

func main() {
	var a, b, c int
	fmt.Scan(&a, &b, &c)
	fmt.Println(c / Min(a, b))
}

func Min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
