package main

import (
	"fmt"
	"sort"
)

func main() {
	var a, b, c int
	fmt.Scan(&a, &b, &c)
	bells := []int{a, b, c}
	sort.Ints(bells)
	fmt.Println(bells[0] + bells[1])
}
