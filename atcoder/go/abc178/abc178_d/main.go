package main

import (
	"fmt"
)

func main() {
	var s int
	fmt.Scan(&s)

	var count int
	for bit := 0; bit < (1 << uint(s-2)); bit++ {
		var sum int
		for j := 0; j < s-2; j++ {
			if (bit>>uint(j))&1 == 1 {
				sum += j + 1
			}
		}
		if sum == s {
			count++
			count %= Mod
		}
	}

	fmt.Println(count % Mod)
}

const Mod = 1000000007

// 7
// {7}
// 3 ~ 4, {3,4}, {4,3}
// => 2

// 9
// {9}
// 3 ~ 6, {3,6}, {4,5}, {5,4}, {6,3}
// 3, 3, 3, {3,3,3}
// => 5

// 12
// {12}
// 3 ~ 9, {3,9}, {4,8}, {5,7}, {6,6}
// {3,4,5}, {4,3,5}, {5,4,3}, {5,3,4}, {4,4,4}
// {3,3,3,3}
// => 11

func Pow(a, n int) int {
	if n == 0 {
		return 1
	}
	v := 1
	for p := n; p > 0; p >>= 1 {
		if p&1 == 1 {
			v = v % Mod * a % Mod
		}
		a = a % Mod * a % Mod
	}
	return v % Mod
}
