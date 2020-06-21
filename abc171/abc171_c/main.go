package main

import (
	"fmt"
	"strings"
)

var LowerAlphabets = [...]string{"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
	"n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"}

func main() {
	var n int
	fmt.Scan(&n)
	var names []string
	for n > 0 {
		names = append([]string{LowerAlphabets[((n - 1) % 26)]}, names...)
		n = (n-1) / 26
	}
	fmt.Println(strings.Join(names, ""))
}
