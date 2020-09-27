package main

import "fmt"

func main() {
	var a, b int
	fmt.Scan(&a, &b)

	sum := 0
	for i := 1; i <= b-a; i++ {
		sum += i
	}
	fmt.Println(sum - b)
}
