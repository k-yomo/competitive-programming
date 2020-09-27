package main

import (
	"fmt"
)

const Mod = 1000000007

func main() {
	var n, a, b int
	fmt.Scan(&n, &a, &b)
	ans := ModPow(2, n) - CombMod(n, a) - CombMod(n, b) - 1
	fmt.Println((ans + 2*Mod) % Mod)
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
