package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	var b string
	fmt.Scan(&b)

	numStrs := strings.Split(b, "")
	var sum int
	for _, numStr := range numStrs {
		num, _ := strconv.Atoi(numStr)
		sum += num
	}

	num, _ := strconv.Atoi(b)
	if num%sum == 0 {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}
