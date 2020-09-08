package main

import (
	"fmt"
	"strings"
)

func main() {
	var s string
	fmt.Scan(&s)
	if len(s) == 0 {
		fmt.Println(strings.ToUpper(string(s[0])))
		return
	}
	fmt.Println(strings.ToUpper(string(s[0])) + strings.ToLower(s[1:]))
}
