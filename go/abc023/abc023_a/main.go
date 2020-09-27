package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	var x string
	fmt.Scan(&x)
	numStrs := strings.Split(x, "")
	var sum int
	for _, numStr := range numStrs {
		num, _ := strconv.Atoi(numStr)
		sum += num
	}
	fmt.Println(sum)
}
