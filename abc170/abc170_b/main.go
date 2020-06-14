package main

import "fmt"

func main() {
	var x, y int
	fmt.Scan(&x, &y)

	if y%2 == 0 && y >= 2*x && y <= 4*x {
		fmt.Println("Yes")
		return
	}
	fmt.Println("No")
}
