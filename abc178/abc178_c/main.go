package main

import (
	"fmt"
)

func main() {
	var n int
	fmt.Scan(&n)
	if n == 1 {
		fmt.Println(0)
		return
	}
	totalComb := Pow(10, n%Mod) % Mod
	onlyOne := Pow(9, n%Mod) % Mod
	excludeBoth := onlyOne - Pow(8, n%Mod)%Mod
	ans := (totalComb - onlyOne - excludeBoth) % Mod
	if ans < 0 {
		ans += Mod
	}
	fmt.Println(ans)
}

const Mod = 1000000007

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
