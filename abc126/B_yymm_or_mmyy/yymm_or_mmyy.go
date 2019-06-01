package main

import (
	"fmt"
	"strconv"
)

// https://atcoder.jp/contests/abc126/tasks/abc126_b
func main() {
	var s string
	fmt.Scan(&s)
	first := strconv.Atoi(s[0:2])
	second := strconv.Atoi(s[2:4])
	ismy := 1 <= first && first <= 12
	isym := 1 <= second && second <= 12
	if isym {
		if ismy {
			fmt.Println("AMBIGUOUS")
		} else {
			fmt.Println("YYMM")
		}
	} else {
		if isym {
			fmt.Println("MMYY")
		} else {
			fmt.Println("NA")
		}
	}
}
