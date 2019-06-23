package utils

import (
	"fmt"
	"math"
)

func ScanNums(len int) (nums []int) {
	var num int
	for i := 0; i < len; i++ {
		fmt.Scan(&num)
		nums = append(nums, num)
	}
	return
}

func AbsInt(n1 int, n2 int) int {
	return int(math.Abs(float64(n1 - n2)))
}

// Gcd returns greatest common divisor
func Gcd(x, y int) int {
	mod := x % y
	if mod > 0 {
		return Gcd(y, mod)
	} else {
		return y
	}
}

// Lcm returns least common multiple
func Lcm(x, y int) int {
	return x * y / Gcd(x, y)
}
