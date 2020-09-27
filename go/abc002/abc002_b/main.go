package main

import (
	"fmt"
	"strings"
)

func main() {
	var s string
	fmt.Scan(&s)
	vowels := []string{"a", "i", "u", "e", "o"}

	for _, letter := range strings.Split(s, "") {
		var isVowel bool
		for _, v := range vowels {
			if letter == v {
				isVowel = true
				break
			}
		}
		if isVowel {
			continue
		}
		fmt.Print(letter)
	}
	fmt.Println("")
}
