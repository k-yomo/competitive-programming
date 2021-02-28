package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)
	var total int
	for i := 1; i <= n; i++ {
		total += i/(n-(i-1))
	}
	fmt.Println()
}
