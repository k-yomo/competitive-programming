package main

import (
	"fmt"
	"strings"
)

func main() {
	var n int
	var s string
	fmt.Scan(&n, &s)

	strs := make([]string, len(s))
	for i, char := range s {
		asciiCode := int(char) + n
		if asciiCode >= 91 {
			asciiCode -= 26
		}
		strs[i] = string(asciiCode)
	}
	fmt.Println(strings.Join(strs, ""))
}
