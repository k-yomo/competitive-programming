package main

import (
	"fmt"
)

func main() {
	var n, m int
	fmt.Scan(&n, &m)
	fmt.Println(CombCount(n,2) +CombCount(m,2))
}

func CombCount(left int, right int) int {
	rightFac := Factorial(right, right)
	if rightFac <= 0 {
		return 0
	}
	return Factorial(left, right) / rightFac
}

func Factorial(n int, times int) (res int) {
	facts := make([]int, n+1)
	if facts[n] != 0 {
		res = facts[n]
		return res
	}

	if n > 0 && times > 0 {
		res = n * Factorial(n-1, times-1)
		return res
	}

	return 1
}
