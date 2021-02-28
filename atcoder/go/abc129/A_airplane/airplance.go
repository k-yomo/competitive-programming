package main

import (
	"fmt"
	"sort"
)

// https://atcoder.jp/contests/abc129/tasks/abc129_a
func main() {
	var p, q, r int
	fmt.Scan(&p, &q, &r)
	hours := []int{p, q, r}
	sort.Ints(hours)
	fmt.Println(hours[0] + hours[1])
}
