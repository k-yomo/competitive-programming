package main

import (
	"fmt"
)

func main() {
	var h int
	fmt.Scan(&h)
	fmt.Println(count(h))
}

func count(h int) int {
	if h == 1 {
		return 1
	}
	return 1 + 2*count(h/2)
}
