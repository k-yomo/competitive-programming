package main

import (
	"fmt"
	"strings"
)

func main()  {
	var s, t string
	fmt.Scan(&s, &t)
	for i := 0; i < len(s); i++ {
		if s[i] == '@' && strings.Contains("atcoder", string(t[i])) {
			continue
		}
		if t[i] == '@' && strings.Contains("atcoder", string(s[i])) {
			continue
		}
		if s[i] != t[i] {
			fmt.Println("You will lose")
			return
		}
	}
	fmt.Println("You can win")
}
