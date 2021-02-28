package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	var n string
	fmt.Scan(&n)

	var repdigits []string
	for i := 0; i < len(n); i++ {
		repdigits = append(repdigits, string(n[0]))
	}
	origNum, _ := strconv.Atoi(n)
	num, _ := strconv.Atoi(strings.Join(repdigits, ""))
	if num < origNum {
		for i := 0; i < len(repdigits); i++ {
			num, _ := strconv.Atoi(repdigits[i])
			repdigits[i] = strconv.Itoa(num + 1)
		}
	}
	fmt.Println(strings.Join(repdigits, ""))
}
