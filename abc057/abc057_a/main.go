package main

import "fmt"

func main() {
	var a, b int
	fmt.Scan(&a, &b)
	if a+b >= 24 {
		fmt.Println(a + b - 24)
	} else {
		fmt.Println(a + b)
	}
}
