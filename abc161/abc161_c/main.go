package main

import (
	"fmt"
)

func main() {
	var n, k int64
	fmt.Scan(&n, &k)
	fmt.Println(Min(AbsInt(n%k, k), n%k))
}

func Min(n1, n2 int64) int64 {
	if n1 < n2 {
		return n1
	}
	return n2
}

func AbsInt(n1, n2 int64) int64 {
	if n1 > n2 {
		return n1 - n2
	}
	return n2 - n1
}
