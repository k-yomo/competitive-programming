package util

import (
	"math"
)

const Mod = 1000000007

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

// CombCount returns combination count(left C right)
func CombCount(left int, right int) int {
	rightFac := Factorial(right, right)
	if rightFac <= 0 {
		return 0
	}
	return Factorial(left, right) / rightFac
}

// Factorial returns factorial (n P times)
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

func Pow64(x, y int64) int64 {
	return int64(math.Pow(float64(x), float64(y)))
}

func Pow(x, y int) int {
	return int(Pow64(int64(x), int64(y)))
}

func ContainsInt(s []int, e int) bool {
	for _, a := range s {
		if a == e {
			return true
		}
	}
	return false
}