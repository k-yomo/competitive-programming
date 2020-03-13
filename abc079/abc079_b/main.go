package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)
	luca := make([]int, n+1)
	luca[0] = 2
	luca[1] = 1
	for i := 2; i <= n; i++ {
		luca[i] = luca[i-1] + luca[i-2]
	}
	fmt.Println(luca[n])
}
