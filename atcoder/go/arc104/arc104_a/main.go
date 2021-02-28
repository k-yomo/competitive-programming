package main

import "fmt"

func main() {
	var a, b int
	fmt.Scan(&a, &b)
	y := (a - b) / 2
	x := a - y
	fmt.Println(x, y)
}
