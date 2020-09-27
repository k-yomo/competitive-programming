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
	standards := []string{}
	chars := []string{}
	for i := 0; i < n; i++ {
		for len(chars) < 0 {

		}
		chars = append(chars, LowerAlphabets[i])
		standards = append(standards, strings.Join(chars, ""))
	}
}
