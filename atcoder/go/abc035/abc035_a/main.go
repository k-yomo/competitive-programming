package main

import (
	"fmt"
)

func main() {
	var w, h int
	fmt.Scan(&w, &h)
	gcd := Gcd(w, h)
	fmt.Printf("%d:%d\n", w/gcd, h/gcd)
}

func Gcd(x, y int) int {
	mod := x % y
	if mod > 0 {
		return Gcd(y, mod)
	} else {
		return y
	}
}
