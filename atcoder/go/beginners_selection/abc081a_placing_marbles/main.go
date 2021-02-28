package main

import (
	"fmt"
)

// https://atcoder.jp/contests/abs/tasks/abc081_a
func main() {
	var num, result int
	fmt.Scanf("%d", &num)
	for result = 0; num > 0; num /= 10 {
		result += num % 10
	}
	fmt.Printf("%d", result)
}
