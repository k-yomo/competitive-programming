package main

import (
	"fmt"
)

func main() {
	var a, b int
	fmt.Scan(&a, &b)
	fmt.Println(Lcm(a, b))
}

func Gcd(x, y int) int {
	mod := x % y
	if mod > 0 {
		return Gcd(y, mod)
	} else {
		return y
	}
}

func Lcm(x, y int) int {
	return x * y / Gcd(x, y)
}
