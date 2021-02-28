package main

import (
	"fmt"
	"strings"
)

func main() {
	var rCount int
	for i := 0; i < 12; i++ {
		var s string
		fmt.Scan(&s)
		if strings.Contains(s, "r") {
			rCount++
		}
	}
	fmt.Println(rCount)
}
