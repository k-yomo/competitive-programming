package main

import "fmt"

func main() {
	var x int
	fmt.Scan(&x)
	n := x % 100 / 5
	if x%100%5 > 0 {
		n++
	}
	if n <= x/100 {
		fmt.Println(1)
	} else {
		fmt.Println(0)
	}
}
