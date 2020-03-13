package main

import (
	"fmt"
	"strings"
)

func main() {
	var c string
	fmt.Scan(&c)
	vowels := "aeiou"
	if strings.Contains(vowels, c) {
		fmt.Println("vowel")
	} else {
		fmt.Println("consonant")
	}
}
