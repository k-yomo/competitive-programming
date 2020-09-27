package main

import (
	"fmt"
	"strings"
)

func main() {
	var d int
	fmt.Scan(&d)
	ans := []string{"Christmas"}
	for i := 0; i < 25 - d; i++ {
		ans = append(ans, "Eve")
	}
	fmt.Println(strings.Join(ans, " "))
}