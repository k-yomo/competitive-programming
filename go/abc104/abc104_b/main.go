package main

import (
	"fmt"
	"strings"
)

func main() {
	var s string
	fmt.Scan(&s)

	if s[0] != 'A' {
		fmt.Println("WA")
		return
	}
	var count int
	for _, c := range s[2 : len(s)-1] {
		if c == 'C' {
			count++
		}
	}
	if count != 1 {
		fmt.Println("WA")
		return
	}

	var upperCount int
	for i := 0; i < len(s); i++ {
		if strings.ToUpper(string(s[i])) == string(s[i]) {
			upperCount++
		}
	}

	if upperCount != 2 {
		fmt.Println("WA")
		return
	}
	fmt.Println("AC")
}
