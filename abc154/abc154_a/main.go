package main

import "fmt"

func main() {
	var s, t, u string
	var a, b int
	fmt.Scan(&s, &t, &a, &b, &u)
	if u == s {
		fmt.Printf("%d %d\n", a-1, b)
	} else {
		fmt.Printf("%d %d\n", a, b-1)
	}
}