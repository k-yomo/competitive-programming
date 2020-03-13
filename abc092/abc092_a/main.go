package main

import "fmt"

func main() {
	var a, b, c, d, total int
	fmt.Scan(&a, &b, &c, &d)
	if a < b {
		total += a
	} else {
		total += b
	}
	if c < d {
		total += c
	} else {
		total += d
	}

	fmt.Println(total)
}
