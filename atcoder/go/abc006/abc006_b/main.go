package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)

	mod := 10007

	tribonacci := make([]int, n+2)
	tribonacci[0], tribonacci[1], tribonacci[2] = 0, 0, 1
	for i := 3; i < n; i++ {
		tribonacci[i] = (tribonacci[i-3]%mod + tribonacci[i-2]%mod + tribonacci[i-1]%mod) % mod
	}
	fmt.Println(tribonacci[n-1])
}
