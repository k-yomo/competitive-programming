package main

import (
	"fmt"
	"strconv"
)

func main() {
	var a, b string
	fmt.Scan(&a, &b)

	num, _ := strconv.Atoi(a+b)
	fmt.Println(num * 2)
}
