package main

import (
	"fmt"
)

// https://atcoder.jp/contests/abc131/tasks/abc131_c
func main() {
	var a, b, c, d int
	fmt.Scan(&a, &b, &c, &d)
	totalNum := b - a + 1
	lcm := c * d / Gcd(c, d)
	var cNum, dNum, eNum int
	if a%c == 0 {
		cNum = a/c-1
	} else {
		cNum = a/c
	}
	if a%d == 0 {
		dNum = a/d-1
	} else {
		dNum = a/d
	}
	if a%lcm == 0 {
		eNum = a/lcm-1
	} else {
		eNum = a/lcm
	}
	ans := totalNum - ((b/c - cNum) + (b/d - dNum) - (b/lcm - eNum))
	fmt.Println(ans)
}

func Gcd(x, y int) int {
	tmp := x % y
	if tmp > 0 {
		return Gcd(y, tmp)
	} else {
		return y
	}
}
