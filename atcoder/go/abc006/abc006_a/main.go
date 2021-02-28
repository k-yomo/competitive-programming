package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main()  {
	var n int
	fmt.Scan(&n)
	if strings.Contains(strconv.Itoa(n), "3") || n%3 == 0 {
		fmt.Println("YES")
	} else {
		fmt.Println("NO")
	}
}
