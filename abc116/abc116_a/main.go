package main

import (
	"fmt"
	"sort"
)

func main() {
	var a, b, c int
	fmt.Scan(&a, &b, &c)
	ints := []int{a, b, c}
	sort.Ints(ints)
	fmt.Println(ints[0] * ints[1] / 2)
}
