package main

import (
	"fmt"
	"sort"
)

func main() {
	var a, b, c int
	fmt.Scan(&a, &b, &c)

	ages := []int{a, b, c}
	sort.Ints(ages)
	fmt.Println(ages[1])
}
