package main

import (
	"fmt"
)

// https://atcoder.jp/contests/abc128/tasks/abc128_a
func main()  {
	var apple, piece int
	fmt.Scan(&apple, &piece)
	fmt.Println((apple * 3 + piece) / 2)
}
