package main

import (
	"fmt"
	"strconv"
)

func main() {
	var n int
	fmt.Scan(&n)
	if s := strconv.Itoa(n); s[0] == s[2] {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
