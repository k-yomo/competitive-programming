package main

import (
	"fmt"
	"strings"
)

// https://atcoder.jp/contests/abc131/tasks/abc131_a
func main() {
	var s string
	fmt.Scan(&s)
	dup := "Good"
	chars := strings.Split(s, "")
	for i := 1; i < len(chars); i++ {
		if chars[i] == chars[i-1] {
			dup = "Bad"
			break
		}
	}
	fmt.Println(dup)
}
