package main

import "fmt"

// https://atcoder.jp/contests/abc126/tasks/abc126_c
func main() {
	var n, k int
	fmt.Scan(&n, &k)
	var winningRate float64
	for i := 1; i <= n; i++ {
		score := i
		rate := float64(1) / float64(n)
		for score < k {
			score *= 2
			rate /= 2
		}
		winningRate += rate
	}
	fmt.Println(winningRate)
}
