package main

import (
	"fmt"
	"strings"
)

func main() {
	var n, k int
	fmt.Scan(&n, &k)
	var count int
	for n > 0 {
		count++
		n = n / k
	}
	fmt.Println(count)
}

func JoinInts(ints []int) string {
	return strings.Trim(strings.Join(strings.Fields(fmt.Sprint(ints)), ""), "[]")
}
