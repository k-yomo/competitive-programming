package main

import (
	"fmt"
)

// https://atcoder.jp/contests/abc126/tasks/abc126_a
func main() {
	var n, k int
	var s string
	fmt.Scan(&n, &k, &s)
	chars := []byte(s)
	chars[k-1] += 32
	fmt.Println(string(chars))
}
