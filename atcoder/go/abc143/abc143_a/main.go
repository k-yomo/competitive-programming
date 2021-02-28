package main

import "fmt"

func main() {
	var a, b int
	fmt.Scan(&a, &b)
	if b*2 > a {
		fmt.Println(0)
	} else {
		fmt.Println(a - b*2)
	}
}
