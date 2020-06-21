package main

import (
	"fmt"
)

const Mod = 1000000007

func main() {
	var a, b, c uint64
	fmt.Scan(&a, &b, &c)
	ans := a % Mod * b % Mod
	fmt.Println((ans * c % Mod) % Mod)
}
