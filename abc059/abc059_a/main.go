package main

import (
	"fmt"
	"strings"
)

func main() {
	var a, b, c string
	fmt.Scan(&a, &b, &c)
	fmt.Println(strings.ToUpper(string(a[0])) + strings.ToUpper(string(b[0])) + strings.ToUpper(string(c[0])))
}
