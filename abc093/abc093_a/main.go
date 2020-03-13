package main

import (
	"fmt"
	"strings"
)

func main() {
	var s string
	fmt.Scan(&s)
	if strings.Contains(s, "a") && strings.Contains(s, "b") && strings.Contains(s, "c") {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
