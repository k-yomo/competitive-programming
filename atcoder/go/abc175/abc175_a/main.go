package main

import (
	"fmt"
	"strings"
)

func main() {
	var s string
	fmt.Scan(&s)
	a := strings.Split(s, "S")
	var count int
	for _, v := range a {
		if v != "" && len(v) > count {
			count = len(v)
		}
	}
	fmt.Println(count)
}

