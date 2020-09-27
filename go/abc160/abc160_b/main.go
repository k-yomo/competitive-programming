package main

import "fmt"

func main() {
	var x int
	fmt.Scan(&x)
	if x%500 == 0 {
		fmt.Println(x / 500 * 1000)
	} else {
		a := (x / 500) * 1000
		x = x % 500
		fmt.Println(a + (x/5)*5)
	}
}
