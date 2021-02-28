package main

import "fmt"

// https://atcoder.jp/contests/abc130/tasks/abc130_a
func main()  {
	var x, a int
	fmt.Scan(&x, &a)
	if x < a {
		fmt.Println(0)
	} else {
		fmt.Println(10)
	}
}
