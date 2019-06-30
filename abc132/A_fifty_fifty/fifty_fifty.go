package main

import "fmt"

// https://atcoder.jp/contests/abc132/tasks/abc132_a
func main() {
	var s string
	fmt.Scan(&s)
	charCount := make(map[int32]int, 0)
	for _, char := range s {
		charCount[char] += 1
	}
	if len(Keys(charCount)) == 2 {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}

func Keys(m map[int32]int) []int32 {
	var ks []int32
	for k := range m {
		ks = append(ks, k)
	}
	return ks
}
