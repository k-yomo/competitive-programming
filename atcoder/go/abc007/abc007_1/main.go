package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)
	if n == 1 {
		fmt.Println(0)
		return
	}

	fmt.Println(n-1)
}