package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)

	var count int
	for i := 1; i <= n; i++ {
		count += (n - 1) / i
	}
	fmt.Println(count)
}
