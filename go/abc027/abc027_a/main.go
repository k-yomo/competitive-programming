package main

import (
	"fmt"
	"sort"
)

func main() {
	var l1, l2, l3 int
	fmt.Scan(&l1, &l2, &l3)

	sides := []int{l1, l2, l3}
	sort.Ints(sides)

	if sides[0] == sides[1] {
		fmt.Println(sides[2])
	} else {
		fmt.Println(sides[0])
	}
}
