package main

import (
	"fmt"
	"strings"
)

func main() {
	var x string
	fmt.Scan(&x)

	fmt.Println(strings.Index("ABCDE", x) + 1)
}
