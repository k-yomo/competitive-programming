package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)
	a := n / 1000
	b := n % 1000 / 100
	c := n % 1000 % 100 / 10
	d := n % 1000 % 100 % 10
	if (a == b && b == c) || b == c && c == d {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
