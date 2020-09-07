package main

import "fmt"

func main() {
	var a, b, c, k, s, t int
	fmt.Scan(&a, &b, &c, &k, &s, &t)

	var sum int
	if s+t >= k {
		sum -= (s+t) * c
	}

	sum += s*a+t*b
	fmt.Println(sum)
}
