package main

import "fmt"

func main() {
	var l, r, d int
	fmt.Scan(&l, &r, &d)
	var count int
	for i := l; i <= r; i++ {
		if i % d == 0 {
			count++
		}
	}
	fmt.Println(count)
}