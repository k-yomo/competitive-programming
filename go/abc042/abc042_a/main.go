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
	if ints[0] == 5 && ints[1] == 5 && ints[2] == 7 {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}
