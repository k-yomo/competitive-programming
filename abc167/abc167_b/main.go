package main

import "fmt"

func main() {
	var a, b, c, k int
	fmt.Scan(&a, &b, &c, &k)
	var sum int
	if a >= k {
		fmt.Println(k)
		return
	} else {
		sum += a
		k = k - a
	}
	if b >= k {
		fmt.Println(sum)
		return
	} else {
		k = k - b
	}
	fmt.Println(sum - k)
}