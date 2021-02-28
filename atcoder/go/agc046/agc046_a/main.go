package main

import "fmt"

func main() {
	var x int
	fmt.Scan(&x)
	fmt.Println(Lcm(x, 360) / x)
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
