package util

import (
	"math"
)

const Mod = 1000000007

func Sum(nums ...int) (sum int) {
	for _, n := range nums {
		sum += n
	}
	return sum
}

func Min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func Max(a, b int) int {
	if a > b {
		return a
	}
	return b
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

func NextCmb(bits int) int {
	lowest := bits & -bits
	newBits := bits + lowest
	newBits |= ((bits & ^newBits) / lowest) >> 1
	return newBits
}

// CombCount returns combination count(left C right)
// for i := 1<<uint(p) - 1; i < 1<<uint(n); i = NextCmb(i) {
// 	happiness := 0
// 	sum := make([]int, m)
// 	for g := 0; g < n; g++ {
// 	// skip if the girl is not included in the group
// 	if i>>uint(g)&1 == 0 {
// 		continue
// 	}
// }
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

func CombMod(left, right int) int {
	if right == 0 {
		return 1
	}
	v := left
	div := right
	for i := 1; i < right; i++ {
		v = v * (left - i) % Mod
		div = div * (right - i) % Mod
	}
	div = ModPow(div, Mod-2)
	return v * div % Mod
}

func ModPow(a, n int) int {
	if n == 0 {
		return 1
	}
	v := 1
	for p := n; p > 0; p >>= 1 {
		if p&1 == 1 {
			v = v * a % Mod
		}
		a = a * a % Mod
	}
	return v
}

func Pow(a, n int) int {
	if n == 0 {
		return 1
	}
	v := 1
	for p := n; p > 0; p >>= 1 {
		if p&1 == 1 {
			v = v * a
		}
		a = a * a
	}
	return v
}

func ContainsInt(s []int, e int) bool {
	for _, a := range s {
		if a == e {
			return true
		}
	}
	return false
}
