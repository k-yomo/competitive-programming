package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	var n int
	fmt.Scan(&n)
	if strings.Contains(strconv.Itoa(n), "9") {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
